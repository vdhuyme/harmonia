use crate::error::AppError;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, Nonce};
use rand::Rng;
use serde::{Deserialize, Serialize};

const NONCE_SIZE: usize = 12; // 96 bits for AES-GCM
const TAG_SIZE: usize = 16; // 128 bits for AES-GCM tag
const KEY_SIZE: usize = 32; // 256 bits for AES-256

/// Encrypted credential with nonce and ciphertext
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EncryptedCredential {
    /// Nonce used for encryption (hex-encoded)
    pub nonce: String,
    /// Ciphertext with tag (hex-encoded)
    pub ciphertext: String,
}

impl EncryptedCredential {
    /// Convert to database-storable format (nonce:ciphertext)
    pub fn to_string(&self) -> String {
        format!("{}:{}", self.nonce, self.ciphertext)
    }

    /// Parse from database format (nonce:ciphertext)
    pub fn from_string(s: &str) -> Result<Self, AppError> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
            return Err(AppError::DecryptionFailed);
        }
        Ok(Self {
            nonce: parts[0].to_string(),
            ciphertext: parts[1].to_string(),
        })
    }
}

/// Encryption handler for provider credentials
pub struct CryptoHandler {
    cipher: Aes256Gcm,
}

impl CryptoHandler {
    /// Create a new crypto handler with a 32-byte key
    pub fn new(key: &[u8; KEY_SIZE]) -> Self {
        use aes_gcm::KeyInit;
        Self {
            cipher: Aes256Gcm::new(key.into()),
        }
    }

    /// Encrypt a plaintext credential
    pub fn encrypt(&self, plaintext: &str) -> Result<EncryptedCredential, AppError> {
        let mut rng = rand::thread_rng();
        let nonce_bytes: [u8; NONCE_SIZE] = rng.gen();
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|_| AppError::EncryptionFailed)?;

        Ok(EncryptedCredential {
            nonce: hex::encode(nonce_bytes),
            ciphertext: hex::encode(ciphertext),
        })
    }

    /// Decrypt a credential
    pub fn decrypt(&self, credential: &EncryptedCredential) -> Result<String, AppError> {
        let nonce_bytes = hex::decode(&credential.nonce)
            .map_err(|_| AppError::DecryptionFailed)?;
        
        if nonce_bytes.len() != NONCE_SIZE {
            return Err(AppError::DecryptionFailed);
        }

        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext_bytes = hex::decode(&credential.ciphertext)
            .map_err(|_| AppError::DecryptionFailed)?;

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext_bytes.as_ref())
            .map_err(|_| AppError::DecryptionFailed)?;

        String::from_utf8(plaintext).map_err(|_| AppError::DecryptionFailed)
    }
}

/// Generate a random encryption key
pub fn generate_key() -> [u8; KEY_SIZE] {
    let mut key = [0u8; KEY_SIZE];
    rand::thread_rng().fill(&mut key);
    key
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        let key = generate_key();
        let handler = CryptoHandler::new(&key);
        let plaintext = "spotify_token_abc123xyz";

        let encrypted = handler
            .encrypt(plaintext)
            .expect("Encryption failed");
        let decrypted = handler
            .decrypt(&encrypted)
            .expect("Decryption failed");

        assert_eq!(decrypted, plaintext);
    }

    #[test]
    fn test_encrypt_produces_different_ciphertext() {
        let key = generate_key();
        let handler = CryptoHandler::new(&key);
        let plaintext = "spotify_token_abc123xyz";

        let encrypted1 = handler.encrypt(plaintext).expect("Encryption failed");
        let encrypted2 = handler.encrypt(plaintext).expect("Encryption failed");

        // Different nonces should produce different ciphertexts
        assert_ne!(encrypted1.ciphertext, encrypted2.ciphertext);
    }

    #[test]
    fn test_decrypt_with_wrong_key_fails() {
        let key1 = generate_key();
        let key2 = generate_key();
        
        let handler1 = CryptoHandler::new(&key1);
        let handler2 = CryptoHandler::new(&key2);
        
        let plaintext = "spotify_token_abc123xyz";
        let encrypted = handler1.encrypt(plaintext).expect("Encryption failed");

        let result = handler2.decrypt(&encrypted);
        assert!(result.is_err());
    }

    #[test]
    fn test_encrypted_credential_serialization() {
        let credential = EncryptedCredential {
            nonce: "abcd1234".to_string(),
            ciphertext: "efgh5678".to_string(),
        };

        let serialized = credential.to_string();
        let deserialized = EncryptedCredential::from_string(&serialized)
            .expect("Deserialization failed");

        assert_eq!(credential, deserialized);
    }

    #[test]
    fn test_encrypted_credential_invalid_format() {
        let result = EncryptedCredential::from_string("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_generate_key_produces_correct_size() {
        let key = generate_key();
        assert_eq!(key.len(), KEY_SIZE);
    }
}
