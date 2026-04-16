use crate::error::AppError;
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};

/// Spotify OAuth configuration
pub struct SpotifyOAuthConfig {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
}

impl SpotifyOAuthConfig {
    /// Create from environment variables
    pub fn from_env() -> Result<Self, AppError> {
        let client_id = std::env::var("SPOTIFY_CLIENT_ID")
            .map_err(|_| AppError::InternalError("SPOTIFY_CLIENT_ID not set".to_string()))?;
        let client_secret = std::env::var("SPOTIFY_CLIENT_SECRET")
            .map_err(|_| AppError::InternalError("SPOTIFY_CLIENT_SECRET not set".to_string()))?;
        let redirect_uri = std::env::var("SPOTIFY_REDIRECT_URI")
            .unwrap_or_else(|_| "http://localhost:3000/auth/spotify/callback".to_string());

        Ok(Self {
            client_id,
            client_secret,
            redirect_uri,
        })
    }

    /// Generate Spotify authorization URL
    pub fn authorization_url(&self, state: &str) -> String {
        format!(
            "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&state={}&scope=user-modify-playback-state user-read-playback-state",
            urlencoding::encode(&self.client_id),
            urlencoding::encode(&self.redirect_uri),
            urlencoding::encode(state)
        )
    }
}

/// Spotify OAuth token response
#[derive(Debug, Deserialize, Serialize)]
pub struct SpotifyTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub refresh_token: Option<String>,
}

/// Spotify authorization request
#[derive(Debug, Deserialize)]
pub struct AuthorizeSpotifyRequest {
    pub code: String,
    pub state: String,
}

/// Spotify authorization response
#[derive(Debug, Serialize)]
pub struct AuthorizeSpotifyResponse {
    pub user_id: String,
    pub access_token: String,
    pub expires_in: i64,
}

/// Request authorization URL from Spotify
#[derive(Debug, Serialize)]
pub struct SpotifyAuthUrlResponse {
    pub authorization_url: String,
    pub state: String,
}

/// Request authorization URL
///
/// TODO: Store state in Redis for CSRF protection
pub async fn spotify_auth_url(
) -> Result<(StatusCode, Json<SpotifyAuthUrlResponse>), AppError> {
    let config = SpotifyOAuthConfig::from_env()?;
    let state = uuid::Uuid::new_v4().to_string();

    let authorization_url = config.authorization_url(&state);

    // TODO: Store state in Redis with expiry
    // redis.setex(format!("oauth_state:{}", state), 600, "true").await?;

    Ok((
        StatusCode::OK,
        Json(SpotifyAuthUrlResponse {
            authorization_url,
            state,
        }),
    ))
}

/// Exchange authorization code for access token
///
/// TODO: Verify state from Redis
/// TODO: Store credentials in database with encryption
pub async fn spotify_callback(
    Json(req): Json<AuthorizeSpotifyRequest>,
) -> Result<(StatusCode, Json<AuthorizeSpotifyResponse>), AppError> {
    // TODO: Verify state from Redis
    // let state_valid = redis.get(format!("oauth_state:{}", req.state)).await?;
    // if state_valid.is_none() {
    //     return Err(AppError::InvalidAuthToken);
    // }

    let config = SpotifyOAuthConfig::from_env()?;
    let client = reqwest::Client::new();

    let params = [
        ("grant_type", "authorization_code"),
        ("code", &req.code),
        ("redirect_uri", &config.redirect_uri),
        ("client_id", &config.client_id),
        ("client_secret", &config.client_secret),
    ];

    let response = client
        .post("https://accounts.spotify.com/api/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| AppError::InternalError(format!("Spotify token exchange failed: {}", e)))?;

    if !response.status().is_success() {
        return Err(AppError::InternalError(
            "Spotify token exchange failed".to_string(),
        ));
    }

    let token_response: SpotifyTokenResponse = response
        .json()
        .await
        .map_err(|e| AppError::InternalError(format!("Failed to parse token response: {}", e)))?;

    // TODO: Get user info from Spotify API
    // TODO: Store encrypted token in database
    // TODO: Return JWT token

    Ok((
        StatusCode::OK,
        Json(AuthorizeSpotifyResponse {
            user_id: "user123".to_string(), // TODO: Get from Spotify API
            access_token: token_response.access_token,
            expires_in: token_response.expires_in,
        }),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spotify_oauth_config() {
        let config = SpotifyOAuthConfig {
            client_id: "test_id".to_string(),
            client_secret: "test_secret".to_string(),
            redirect_uri: "http://localhost:3000/callback".to_string(),
        };

        assert_eq!(config.client_id, "test_id");
    }

    #[test]
    fn test_spotify_authorization_url_format() {
        let config = SpotifyOAuthConfig {
            client_id: "client123".to_string(),
            client_secret: "secret123".to_string(),
            redirect_uri: "http://localhost:3000/callback".to_string(),
        };

        let url = config.authorization_url("state123");
        
        assert!(url.contains("https://accounts.spotify.com/authorize"));
        assert!(url.contains("client_id=client123"));
        assert!(url.contains("response_type=code"));
        assert!(url.contains("state=state123"));
    }

    #[test]
    fn test_spotify_token_response_parsing() {
        let json = r#"{
            "access_token": "token123",
            "token_type": "Bearer",
            "expires_in": 3600,
            "refresh_token": "refresh123"
        }"#;

        let response: SpotifyTokenResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.access_token, "token123");
        assert_eq!(response.expires_in, 3600);
    }

    #[test]
    fn test_authorize_spotify_request() {
        let req = AuthorizeSpotifyRequest {
            code: "code123".to_string(),
            state: "state123".to_string(),
        };

        assert_eq!(req.code, "code123");
        assert_eq!(req.state, "state123");
    }
}
