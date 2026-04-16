use api::{jwt::Claims, middleware::AuthUser, AppState};
use chrono::Utc;

#[test]
fn test_jwt_claims_creation() {
    let claims = Claims::new("user123".to_string(), "user".to_string(), 24);
    assert_eq!(claims.sub, "user123");
    assert_eq!(claims.role, "user");
    assert!(!claims.is_expired());
}

#[test]
fn test_jwt_claims_with_room_scope() {
    let claims = Claims::new("user123".to_string(), "user".to_string(), 24)
        .with_room("room456".to_string());
    assert_eq!(claims.room_id, Some("room456".to_string()));
}

#[test]
fn test_jwt_handler_token_creation() {
    let secret = b"test-secret-key-at-least-32-bytes-long-for-testing";
    let handler = api::jwt::JwtHandler::new(secret);
    let claims = Claims::new("user123".to_string(), "admin".to_string(), 1);

    let token = handler
        .create_token(&claims)
        .expect("Failed to create token");
    assert!(!token.is_empty());
    assert!(token.contains('.'));
}

#[test]
fn test_jwt_handler_token_verification() {
    let secret = b"test-secret-key-at-least-32-bytes-long-for-testing";
    let handler = api::jwt::JwtHandler::new(secret);
    let original = Claims::new("user123".to_string(), "admin".to_string(), 1);

    let token = handler
        .create_token(&original)
        .expect("Failed to create token");
    let verified = handler
        .verify_token(&token)
        .expect("Failed to verify token");

    assert_eq!(verified.sub, original.sub);
    assert_eq!(verified.role, original.role);
}

#[test]
fn test_auth_user_admin_check() {
    let admin = AuthUser {
        user_id: "user1".to_string(),
        role: "admin".to_string(),
        room_id: None,
    };
    assert!(admin.is_admin());

    let user = AuthUser {
        user_id: "user2".to_string(),
        role: "user".to_string(),
        room_id: None,
    };
    assert!(!user.is_admin());
}

#[test]
fn test_auth_user_room_access() {
    let scoped = AuthUser {
        user_id: "user1".to_string(),
        role: "user".to_string(),
        room_id: Some("room1".to_string()),
    };
    assert!(scoped.can_access_room("room1"));
    assert!(!scoped.can_access_room("room2"));

    let admin = AuthUser {
        user_id: "admin1".to_string(),
        role: "admin".to_string(),
        room_id: None,
    };
    assert!(admin.can_access_room("any_room"));
}

#[test]
fn test_crypto_encryption_decryption() {
    let key = api::crypto::generate_key();
    let handler = api::crypto::CryptoHandler::new(&key);
    let plaintext = "spotify_token_abc123";

    let encrypted = handler.encrypt(plaintext).expect("Failed to encrypt");
    let decrypted = handler.decrypt(&encrypted).expect("Failed to decrypt");

    assert_eq!(decrypted, plaintext);
}

#[test]
fn test_crypto_wrong_key_fails() {
    let key1 = api::crypto::generate_key();
    let key2 = api::crypto::generate_key();

    let handler1 = api::crypto::CryptoHandler::new(&key1);
    let handler2 = api::crypto::CryptoHandler::new(&key2);

    let plaintext = "spotify_token_abc123";
    let encrypted = handler1.encrypt(plaintext).expect("Failed to encrypt");

    let result = handler2.decrypt(&encrypted);
    assert!(result.is_err());
}

#[test]
fn test_encrypted_credential_serialization() {
    let credential = api::crypto::EncryptedCredential {
        nonce: "abcd1234567890ab".to_string(),
        ciphertext: "efgh5678".to_string(),
    };

    let serialized = credential.to_string();
    let deserialized =
        api::crypto::EncryptedCredential::from_string(&serialized)
            .expect("Failed to deserialize");

    assert_eq!(credential, deserialized);
}

#[test]
fn test_app_state_from_env() {
    let state = AppState::from_env();
    assert!(!state.jwt_secret.is_empty());
}
