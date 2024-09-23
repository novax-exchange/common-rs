use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::error::Error;

pub fn key_pair() -> Result<(RsaPrivateKey, RsaPublicKey ), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key: RsaPrivateKey = RsaPrivateKey::new(&mut rng, bits)?;
    let pub_key = RsaPublicKey::from(&priv_key);
    Ok((priv_key, pub_key))
}

pub fn encrypt(buf: Vec<u8>, pub_key: RsaPublicKey) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &buf[..]);
    Ok(enc_data?)
}

pub fn decrypt(enc_data: Vec<u8>, priv_key: RsaPrivateKey) -> Result<Vec::<u8>,  Box<dyn Error>> {
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data)?;
    Ok(dec_data)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_encrypt_decrypt_the_same() -> Result<(), Box<dyn Error>> {
        let (p0, p1) = key_pair()?;
        let data_str = b"hello testing!!!";
        let enc_data = encrypt(data_str.to_vec(), p1)?;
        let dec_data = decrypt(enc_data, p0)?;
        assert_eq!(dec_data,  data_str, "decrypt data string same as input string" );
        Ok(())
    }
}
