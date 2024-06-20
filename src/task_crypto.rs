use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use hex::{decode, encode};
use rand::rngs::OsRng;
use rand::Rng;
pub fn encrypt(data: &[u8], key: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let nonce = Nonce::from_slice(&OsRng.gen::<[u8; 12]>()); // 96-bits; unique per message
    let ciphertext = cipher.encrypt(nonce, data).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    // Combine nonce and ciphertext
    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);

    Ok(encode(combined))
}

pub fn decrypt(encrypted_data: &str, key: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let key = Key::from_slice(key);
    let cipher = Aes256Gcm::new(key);

    let encrypted_data = decode(encrypted_data)?;
    let (nonce, ciphertext) = encrypted_data.split_at(12); // 12 bytes for the nonce

    let nonce = Nonce::from_slice(nonce); // 96-bits
    let plaintext = cipher.decrypt(nonce, ciphertext)?;

    Ok(plaintext)
}