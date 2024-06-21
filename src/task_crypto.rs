use aes_gcm::{Aes256Gcm, Key, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use hex::{decode, encode};
use rand::rngs::OsRng;
use rand::Rng;
use sha2::{Sha256, Digest};
use crate::error::TaskError;

fn derive_key(password: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(password);
    let result = hasher.finalize();
    result.into()


}

pub fn encrypt(data: &[u8], pw: &[u8]) -> Result<String, TaskError> {
    let key = derive_key(pw);
    let key = Key::<Aes256Gcm>::from_slice(&key);
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

pub fn decrypt(encrypted_data: &str, pw: &[u8]) -> Result<Vec<u8>, TaskError> {
    let key = derive_key(pw);
    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(key);

    let encrypted_data = decode(encrypted_data)?;
    let (nonce, ciphertext) = encrypted_data.split_at(12);

    let nonce = Nonce::from_slice(nonce);
    let plaintext = cipher.decrypt(nonce, ciphertext)?;

    Ok(plaintext)
}