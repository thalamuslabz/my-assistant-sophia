use aes_gcm::{
    aead::{Aead, KeyInit, OsRng},
    Aes256Gcm, Nonce, Key
};
use rand::RngCore;
use std::error::Error;

const NONCE_SIZE: usize = 12; // 96 bits for GCM

/// Derives a master encryption key from machine ID and application salt
pub fn derive_master_key() -> Result<[u8; 32], Box<dyn Error>> {
    // Get machine-specific identifier
    let machine_id = machine_uid::get()
        .map_err(|e| format!("Failed to get machine ID: {}", e))?;
    
    // Application salt (hardcoded)
    let app_salt = b"sophia-assistant-v1.2-encryption-key";
    
    // Combine machine ID and salt
    let combined = format!("{}{}", machine_id, String::from_utf8_lossy(app_salt));
    
    // Use SHA-256 to derive 256-bit key
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(combined.as_bytes());
    let result = hasher.finalize();
    
    let mut key = [0u8; 32];
    key.copy_from_slice(&result);
    
    log::info!("Master key derived from machine ID");
    Ok(key)
}

/// Encrypts plaintext using AES-256-GCM
/// Returns: (nonce || ciphertext) as a single Vec<u8>
pub fn encrypt(plaintext: &[u8], master_key: &[u8; 32]) -> Result<Vec<u8>, Box<dyn Error>> {
    let key = Key::<Aes256Gcm>::from_slice(master_key);
    let cipher = Aes256Gcm::new(key);
    
    // Generate random nonce
    let mut nonce_bytes = [0u8; NONCE_SIZE];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    
    // Encrypt
    let ciphertext = cipher.encrypt(nonce, plaintext)
        .map_err(|e| format!("Encryption failed: {}", e))?;
    
    // Prepend nonce to ciphertext
    let mut result = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    
    log::debug!("Encrypted {} bytes -> {} bytes (with nonce)", plaintext.len(), result.len());
    Ok(result)
}

/// Decrypts ciphertext using AES-256-GCM
/// Expects: (nonce || ciphertext) as input
pub fn decrypt(encrypted_data: &[u8], master_key: &[u8; 32]) -> Result<Vec<u8>, Box<dyn Error>> {
    if encrypted_data.len() < NONCE_SIZE {
        return Err("Encrypted data too short".into());
    }
    
    // Extract nonce and ciphertext
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_bytes);
    
    let key = Key::<Aes256Gcm>::from_slice(master_key);
    let cipher = Aes256Gcm::new(key);
    
    // Decrypt
    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;
    
    log::debug!("Decrypted {} bytes -> {} bytes", encrypted_data.len(), plaintext.len());
    Ok(plaintext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_master_key_derivation() {
        let key1 = derive_master_key().unwrap();
        let key2 = derive_master_key().unwrap();
        
        // Same machine should produce same key
        assert_eq!(key1, key2);
        assert_eq!(key1.len(), 32);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let master_key = derive_master_key().unwrap();
        let plaintext = b"Hello, World! This is a secret API key.";
        
        // Encrypt
        let encrypted = encrypt(plaintext, &master_key).unwrap();
        
        // Should be longer due to nonce and auth tag
        assert!(encrypted.len() > plaintext.len());
        
        // Decrypt
        let decrypted = decrypt(&encrypted, &master_key).unwrap();
        
        // Should match original
        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_different_nonces() {
        let master_key = derive_master_key().unwrap();
        let plaintext = b"Same plaintext";
        
        let encrypted1 = encrypt(plaintext, &master_key).unwrap();
        let encrypted2 = encrypt(plaintext, &master_key).unwrap();
        
        // Different nonces should produce different ciphertexts
        assert_ne!(encrypted1, encrypted2);
        
        // But both should decrypt to same plaintext
        let decrypted1 = decrypt(&encrypted1, &master_key).unwrap();
        let decrypted2 = decrypt(&encrypted2, &master_key).unwrap();
        assert_eq!(decrypted1, plaintext);
        assert_eq!(decrypted2, plaintext);
    }

    #[test]
    fn test_wrong_key_fails() {
        let master_key = derive_master_key().unwrap();
        let wrong_key = [0u8; 32]; // All zeros
        
        let plaintext = b"Secret data";
        let encrypted = encrypt(plaintext, &master_key).unwrap();
        
        // Decryption with wrong key should fail
        let result = decrypt(&encrypted, &wrong_key);
        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_data_fails() {
        let master_key = derive_master_key().unwrap();
        let plaintext = b"Secret data";
        
        let mut encrypted = encrypt(plaintext, &master_key).unwrap();
        
        // Tamper with the ciphertext
        if let Some(byte) = encrypted.last_mut() {
            *byte ^= 0xFF; // Flip all bits in last byte
        }
        
        // Decryption should fail due to authentication tag
        let result = decrypt(&encrypted, &master_key);
        assert!(result.is_err());
    }
}
