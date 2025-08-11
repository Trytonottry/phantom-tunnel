use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use rand::RngCore;

pub fn encrypt( &[u8], key: &[u8; 32]) -> Vec<u8> {
    let cipher = Aes256Gcm::new(key.into());
    let mut nonce = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    let nonce = Nonce::from_slice(&nonce);
    let ciphertext = cipher.encrypt(nonce, data.as_ref()).unwrap();
    [&nonce[..], &ciphertext[..]].concat()
}