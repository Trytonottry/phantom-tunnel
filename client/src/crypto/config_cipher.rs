use sodiumoxide::crypto::secretbox;
use std::fs;

pub fn decrypt_config(
    encrypted_path: &str,
    key: &[u8; 32],
) -> Result<String, Box<dyn std::error::Error>> {
    let data = fs::read(encrypted_path)?;
    let nonce = secretbox::Nonce::from_slice(&data[..24]).unwrap();
    let ciphertext = &data[24..];
    let plaintext = secretbox::open(ciphertext, &nonce, key)
        .map_err(|_| "decrypt failed")?;
    Ok(String::from_utf8(plaintext)?)
}

pub fn encrypt_config(
    plaintext: &str,
    key: &[u8; 32],
) -> Vec<u8> {
    let mut nonce = [0u8; 24];
    sodiumoxide::randombytes::randomize(&mut nonce);
    let cipher = secretbox::seal(plaintext.as_bytes(), &nonce.into(), key);
    [&nonce[..], &cipher[..]].concat()
}