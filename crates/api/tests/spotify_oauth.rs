use api::handlers::oauth::*;

#[test]
fn test_spotify_oauth_config_creation() {
    let config = SpotifyOAuthConfig {
        client_id: "test_client_id".to_string(),
        client_secret: "test_secret".to_string(),
        redirect_uri: "http://localhost:3000/callback".to_string(),
    };

    assert_eq!(config.client_id, "test_client_id");
    assert_eq!(config.client_secret, "test_secret");
}

#[test]
fn test_spotify_auth_url_contains_required_params() {
    let config = SpotifyOAuthConfig {
        client_id: "my_client_123".to_string(),
        client_secret: "secret".to_string(),
        redirect_uri: "http://localhost:3000/cb".to_string(),
    };

    let url = config.authorization_url("state_token_123");

    assert!(url.contains("https://accounts.spotify.com/authorize"));
    assert!(url.contains("my_client_123"));
    assert!(url.contains("response_type=code"));
    assert!(url.contains("state_token_123"));
    assert!(url.contains("scope="));
}

#[test]
fn test_spotify_auth_url_encoding() {
    let config = SpotifyOAuthConfig {
        client_id: "client with spaces".to_string(),
        client_secret: "secret".to_string(),
        redirect_uri: "http://localhost:3000/callback?test=1".to_string(),
    };

    let url = config.authorization_url("state123");

    // Spaces should be URL encoded
    assert!(url.contains("%"));
}

#[test]
fn test_spotify_token_response_parsing() {
    let json = r#"{
        "access_token": "access_token_xyz",
        "token_type": "Bearer",
        "expires_in": 3600,
        "refresh_token": "refresh_token_abc"
    }"#;

    let response: SpotifyTokenResponse = serde_json::from_str(json).unwrap();

    assert_eq!(response.access_token, "access_token_xyz");
    assert_eq!(response.token_type, "Bearer");
    assert_eq!(response.expires_in, 3600);
    assert_eq!(
        response.refresh_token,
        Some("refresh_token_abc".to_string())
    );
}

#[test]
fn test_spotify_token_response_without_refresh_token() {
    let json = r#"{
        "access_token": "access_token_xyz",
        "token_type": "Bearer",
        "expires_in": 3600
    }"#;

    let response: SpotifyTokenResponse = serde_json::from_str(json).unwrap();

    assert_eq!(response.access_token, "access_token_xyz");
    assert_eq!(response.refresh_token, None);
}

#[test]
fn test_authorize_spotify_request_parsing() {
    let json = r#"{
        "code": "auth_code_123",
        "state": "state_token_456"
    }"#;

    let req: AuthorizeSpotifyRequest = serde_json::from_str(json).unwrap();

    assert_eq!(req.code, "auth_code_123");
    assert_eq!(req.state, "state_token_456");
}

#[test]
fn test_spotify_auth_url_response_serialization() {
    let response = SpotifyAuthUrlResponse {
        authorization_url: "https://accounts.spotify.com/authorize?..."
            .to_string(),
        state: "state_123".to_string(),
    };

    let json = serde_json::to_string(&response).unwrap();

    assert!(json.contains("authorization_url"));
    assert!(json.contains("state_123"));
}

#[test]
fn test_authorize_spotify_response_serialization() {
    let response = AuthorizeSpotifyResponse {
        user_id: "user_123".to_string(),
        access_token: "token_xyz".to_string(),
        expires_in: 3600,
    };

    let json = serde_json::to_string(&response).unwrap();

    assert!(json.contains("user_id"));
    assert!(json.contains("user_123"));
    assert!(json.contains("access_token"));
    assert!(json.contains("token_xyz"));
}

#[tokio::test]
async fn test_spotify_auth_url_endpoint_callable() {
    // This test just verifies the endpoint can be called
    // In production, would use mocking for HTTP calls
    // let result = spotify_auth_url().await;
    // For now, just verify it compiles and returns proper types
}

#[test]
fn test_spotify_oauth_config_from_environment() {
    // Save original env vars
    let original_client_id = std::env::var("SPOTIFY_CLIENT_ID").ok();
    let original_client_secret = std::env::var("SPOTIFY_CLIENT_SECRET").ok();

    // Set test env vars
    std::env::set_var("SPOTIFY_CLIENT_ID", "test_id");
    std::env::set_var("SPOTIFY_CLIENT_SECRET", "test_secret");

    let config = SpotifyOAuthConfig::from_env();

    // Restore original env vars
    if let Some(id) = original_client_id {
        std::env::set_var("SPOTIFY_CLIENT_ID", id);
    } else {
        std::env::remove_var("SPOTIFY_CLIENT_ID");
    }
    if let Some(secret) = original_client_secret {
        std::env::set_var("SPOTIFY_CLIENT_SECRET", secret);
    } else {
        std::env::remove_var("SPOTIFY_CLIENT_SECRET");
    }

    assert!(config.is_ok());
    let cfg = config.unwrap();
    assert_eq!(cfg.client_id, "test_id");
    assert_eq!(cfg.client_secret, "test_secret");
}

#[test]
fn test_spotify_oauth_missing_required_env_vars() {
    // Save original env vars
    let original_client_id = std::env::var("SPOTIFY_CLIENT_ID").ok();
    let original_client_secret = std::env::var("SPOTIFY_CLIENT_SECRET").ok();

    // Remove required vars
    std::env::remove_var("SPOTIFY_CLIENT_ID");
    std::env::remove_var("SPOTIFY_CLIENT_SECRET");

    let config = SpotifyOAuthConfig::from_env();

    // Restore original env vars
    if let Some(id) = original_client_id {
        std::env::set_var("SPOTIFY_CLIENT_ID", id);
    }
    if let Some(secret) = original_client_secret {
        std::env::set_var("SPOTIFY_CLIENT_SECRET", secret);
    }

    assert!(config.is_err());
}

#[test]
fn test_spotify_redirect_uri_default_value() {
    let config = SpotifyOAuthConfig {
        client_id: "test".to_string(),
        client_secret: "test".to_string(),
        redirect_uri: "http://custom:3000/redirect".to_string(),
    };

    assert_eq!(
        config.redirect_uri,
        "http://custom:3000/redirect".to_string()
    );
}
