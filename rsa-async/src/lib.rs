use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::error::Error;
use novax_tokio::tokio as tokio;
use tokio::sync::oneshot;
use std::thread;

pub async fn key_pair() -> Result<(RsaPrivateKey, RsaPublicKey ), Box<dyn Error>> {
    let (tx, rx) = oneshot::channel::<(Option<RsaPrivateKey>, Option<RsaPublicKey> )>();
    std::thread::spawn(move || {
        let mut rng = rand::thread_rng();
        let bits = 2048;
        let _  = tx.send(
            match RsaPrivateKey::new(&mut rng, bits) {
                Ok(priv_key) => {
                    let pub_key = RsaPublicKey::from(&priv_key);
                    ( Some(priv_key), Some(pub_key))
                }, 
                Err(e) => {
                    eprintln!("error create private key {:?}", e);
                    ( None, None)
                }
            }
        );
    });
    let keys =  rx.await?;
    if let( Some(priv_key), Some(pub_key) ) = keys {
        return  Ok((priv_key, pub_key))
    }
    Err(rsa::Error::InputNotHashed.into())
}

pub async fn encrypt(buf: Vec<u8>, pub_key: RsaPublicKey) -> Result<Vec<u8>, Box<dyn Error>> {
    let (tx, rx) = oneshot::channel::<Result<Vec<u8>, rsa::Error>>();
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        let rslt = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &buf[..]);
        let _ = tx.send(rslt);
    });
    let rslt = rx.await?;
    Ok(rslt?)
}

pub async fn decrypt(enc_data: Vec<u8>, priv_key: RsaPrivateKey) -> Result<Vec::<u8>,  Box<dyn Error>> {
    let (tx, rx) = oneshot::channel::<Result<Vec<u8>, rsa::Error>>();
    thread::spawn(move || {
        let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data);
        let _ = tx.send(dec_data);
    });
    let rslt = rx.await?;
    Ok(rslt?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn should_encrypt_decrypt_async_the_same() -> Result<(), Box<dyn Error>> {
        let (p0, p1) = key_pair().await?;
        let data_str = b"hello testing async!!!";
        let enc_data = encrypt(data_str.to_vec(), p1).await?;
        let dec_data = decrypt(enc_data, p0).await?;
        assert_eq!(dec_data,  data_str, "decrypt data string same as input string" );
        Ok(())
    }
}
