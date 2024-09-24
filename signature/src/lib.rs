use std::error::Error;
use rsa::{RsaPrivateKey, RsaPublicKey};
use rsa::pkcs1v15::Pkcs1v15Sign;
use k256::elliptic_curve::{ PublicKey, SecretKey};
use k256::ecdsa::{SigningKey, Signature, signature::Signer};
use k256::ecdsa::{VerifyingKey, signature::Verifier};
// rsa
pub fn rsa_sign_keys() -> Result<(RsaPrivateKey, RsaPublicKey), Box<dyn Error>>  {
    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, 2048)?;
    let pub_key = priv_key.to_public_key();
    Ok((priv_key, pub_key ))
}

pub fn sign_rsa(priv_key: &RsaPrivateKey, signature: Vec<u8>) -> Result< Vec::<u8>, Box<dyn Error> >  {
    let signature = priv_key.sign(Pkcs1v15Sign::new_unprefixed(), &signature)?;
    Ok( signature )
}

pub fn verify_rsa(orig_msg: Vec::<u8>,  sign_msg: Vec::<u8>, pub_key: &RsaPublicKey) -> Result<(), Box<dyn Error>> {
    let rslt = pub_key.verify(Pkcs1v15Sign::new_unprefixed(), &orig_msg, &sign_msg)?;
    Ok(rslt)
}

// ecdsa
pub fn ecdsa_sign_keys() -> Result<(SecretKey<k256::Secp256k1>, PublicKey<k256::Secp256k1>), Box<dyn Error>>  {
    let mut rng = rand::thread_rng();
    let secret_key =  k256::elliptic_curve::SecretKey::random(&mut rng);
    let pub_key = secret_key.public_key();
    Ok((secret_key, pub_key ))
}

pub fn sign_ecdsa(sec_key: &SecretKey<k256::Secp256k1>, sign_msg: Vec<u8>) -> Result<Signature, Box<dyn Error> >  {
    let signing: SigningKey = SigningKey::from(sec_key);
    let signature: Signature = signing.sign( &sign_msg );
    Ok( signature )
}

pub fn verify_ecdsa(signature: &Signature, orig_msg: Vec::<u8>, sec_key: &SecretKey<k256::Secp256k1>) -> Result<(), Box<dyn Error>> {
    let signing: SigningKey = SigningKey::from(sec_key);
    let verify_key = VerifyingKey::from(&signing);
    let rslt = verify_key.verify(&orig_msg, signature)?;
    Ok(rslt)
}


// signing ...k256::Secp256k1

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rsa_signing_should_verified() -> Result<(), Box<dyn Error>> {
        let key_pairs = rsa_sign_keys()?;
        let msg = b"sample signing message";
        let rslt = sign_rsa(&key_pairs.0,  msg.to_vec())?;
        let rslt_2 = verify_rsa(msg.to_vec(), rslt.to_vec(), &key_pairs.1);
        assert_eq!(rslt.len(), 256, "signing is ok");
        assert_eq!(rslt_2?, (), "verify is ok");
        Ok(())
    }

    #[test]
    fn ecdsa_signing_should_verified() -> Result<(), Box<dyn Error>> {
        let key_pairs = ecdsa_sign_keys()?;
        let msg = b"sample signing ecdsa message";
        let sig = sign_ecdsa(&key_pairs.0, msg.to_vec())?;
        assert_eq!(sig.to_bytes().len(), 64, "ecdsa signing is ok");
        let rslt = verify_ecdsa(&sig, msg.to_vec(), &key_pairs.0)?;
        assert_eq!(rslt, (), "verify is ok ecdsa");
        Ok(())
    }
}
