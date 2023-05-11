use rsa::{pkcs1v15::{SigningKey, VerifyingKey}, sha2::Sha256};
use std::fs::File;
use rsa::signature::Signer;

use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rand;



pub fn signature() {
    /* 
    let mut rng = rand::thread_rng();
    // Carregar o certificado e a chave privada RSA
     // Lê a chave privada RSA de 1024 bits de um arquivo
     let bits = 1024;
    let private_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let signing_key = SigningKey::<Sha256>::new(private_key);
    
    // Sign
    let data = b"hello world";
    let signature = signing_key.sign( );
    assert_ne!(signature.to_bytes().as_ref(), data.as_slice());
    */
}