use aes_gcm::{aead::Aead, Aes256Gcm, Key, KeyInit, Nonce};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use domain::error::{DomainError, DomainResult};
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // User ID
    pub exp: usize,  // Expiration time
}

pub struct SecurityService {
    jwt_secret: String,
    encryption_key: Vec<u8>,
}

impl SecurityService {
    pub fn new(jwt_secret: String, encryption_key: Vec<u8>) -> Self {
        Self {
            jwt_secret,
            encryption_key,
        }
    }

    /// Generates a JWT access token for a user
    pub fn create_token(&self, user_id: &str) -> DomainResult<String> {
        let expiration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            + 3600; // 1 hour expiration

        let claims = Claims {
            sub: user_id.to_owned(),
            exp: expiration as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    /// Validates a JWT token and returns the user ID
    pub fn validate_token(&self, token: &str) -> DomainResult<String> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )
        .map_err(|e| {
            DomainError::ValidationError(format!("Invalid token: {}", e))
        })?;

        Ok(token_data.claims.sub)
    }

    /// Encrypts data using AES-256-GCM
    pub fn encrypt(&self, plaintext: &str) -> DomainResult<String> {
        let key = Key::<Aes256Gcm>::from_slice(&self.encryption_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(b"unique nonce 12"); // In production, use a random nonce and store it with the ciphertext

        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(BASE64.encode(ciphertext))
    }

    /// Decrypts data using AES-256-GCM
    pub fn decrypt(&self, ciphertext_base64: &str) -> DomainResult<String> {
        let ciphertext = BASE64.decode(ciphertext_base64).map_err(|e| {
            DomainError::ValidationError(format!("Invalid base64: {}", e))
        })?;

        let key = Key::<Aes256Gcm>::from_slice(&self.encryption_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(b"unique nonce 12");

        let plaintext = cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        String::from_utf8(plaintext)
            .map_err(|e| DomainError::ValidationError(e.to_string()))
    }
}
