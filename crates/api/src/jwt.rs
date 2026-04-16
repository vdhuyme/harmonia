use crate::error::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT payload claims
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Claims {
    /// User ID (UUID)
    pub sub: String,
    /// User role (e.g., "user", "admin")
    pub role: String,
    /// Room ID (optional, for room-specific tokens)
    pub room_id: Option<String>,
    /// Issued at timestamp
    pub iat: i64,
    /// Expiration timestamp
    pub exp: i64,
    /// JWT ID (unique token identifier)
    pub jti: String,
}

impl Claims {
    /// Create new claims for a user
    pub fn new(user_id: String, role: String, expires_in_hours: i64) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::hours(expires_in_hours)).timestamp();
        
        Self {
            sub: user_id,
            role,
            room_id: None,
            iat: now.timestamp(),
            exp,
            jti: Uuid::new_v4().to_string(),
        }
    }

    /// Create claims with room scope
    pub fn with_room(mut self, room_id: String) -> Self {
        self.room_id = Some(room_id);
        self
    }

    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        Utc::now().timestamp() > self.exp
    }
}

/// JWT token handler
pub struct JwtHandler {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl JwtHandler {
    /// Create a new JWT handler with a secret
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    /// Create a token from claims
    pub fn create_token(&self, claims: &Claims) -> Result<String, AppError> {
        encode(&Header::default(), claims, &self.encoding_key)
            .map_err(|_| AppError::TokenCreationFailed)
    }

    /// Verify and decode a token
    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())
            .map(|data| data.claims)
            .map_err(|_| AppError::InvalidToken)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_claims() {
        let claims = Claims::new("user123".to_string(), "user".to_string(), 24);
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.role, "user");
        assert!(claims.room_id.is_none());
        assert!(!claims.is_expired());
    }

    #[test]
    fn test_create_claims_with_room() {
        let claims = Claims::new("user123".to_string(), "user".to_string(), 24)
            .with_room("room456".to_string());
        assert_eq!(claims.sub, "user123");
        assert_eq!(claims.room_id, Some("room456".to_string()));
    }

    #[test]
    fn test_token_creation_and_verification() {
        let secret = b"test-secret-key-at-least-32-bytes-long";
        let handler = JwtHandler::new(secret);
        let claims = Claims::new("user123".to_string(), "admin".to_string(), 1);

        let token = handler.create_token(&claims).expect("Failed to create token");
        let verified = handler.verify_token(&token).expect("Failed to verify token");

        assert_eq!(verified.sub, claims.sub);
        assert_eq!(verified.role, claims.role);
        assert_eq!(verified.jti, claims.jti);
    }

    #[test]
    fn test_token_verification_fails_with_wrong_secret() {
        let handler1 = JwtHandler::new(b"secret-key-1234567890123456789");
        let handler2 = JwtHandler::new(b"different-secret-key-1234567890");
        let claims = Claims::new("user123".to_string(), "user".to_string(), 1);

        let token = handler1.create_token(&claims).expect("Failed to create token");
        let result = handler2.verify_token(&token);

        assert!(result.is_err());
    }

    #[test]
    fn test_claims_expiration() {
        let secret = b"test-secret-key-at-least-32-bytes-long";
        let handler = JwtHandler::new(secret);
        
        // Create claims that expire in the past
        let mut claims = Claims::new("user123".to_string(), "user".to_string(), -1);
        // Manually set exp to past
        claims.exp = (Utc::now() - Duration::hours(1)).timestamp();

        assert!(claims.is_expired());
    }
}
