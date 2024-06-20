use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use hex::{decode, encode};
use rand::rngs::OsRng;
use rand::Rng;
use crate::error::TaskError;
pub fn encrypt(data: &[u8], key: &[u8]) -> Result<String, TaskError> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);
    
    let mut nonce = [0u8; 12];
    OsRng.fill(&mut nonce);
    let nonce = Nonce::from_slice(&nonce);
    
    let ciphertext = cipher.encrypt(nonce, data)?;
    // Combine nonce and ciphertext
    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);

    Ok(encode(combined))
}

pub fn decrypt(encrypted_data: &str, key: &[u8]) -> Result<Vec<u8>, TaskError> {
    let key = Key::<Aes256Gcm>::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let encrypted_data = decode(encrypted_data)?;
    let (nonce, ciphertext) = encrypted_data.split_at(12);

    let nonce = Nonce::from_slice(nonce);
    let plaintext = cipher.decrypt(nonce, ciphertext)?;

    Ok(plaintext)
}