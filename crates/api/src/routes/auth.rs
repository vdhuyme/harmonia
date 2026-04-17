use crate::dto::{
    AuthResponseDto, SpotifyAuthRequestDto, YouTubeAuthRequestDto,
};
use crate::error::AppError;
use crate::extractors::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/spotify/url", get(spotify_auth_url))
        .route("/auth/spotify/callback", post(spotify_auth_callback))
        .route("/auth/youtube/url", get(youtube_auth_url))
        .route("/auth/youtube/callback", post(youtube_auth_callback))
}

async fn spotify_auth_url(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let auth_url = format!(
        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&scope=user-read-private%20user-read-email%20streaming%20user-modify-playback-state",
        state.config.spotify_client_id,
        state.config.spotify_redirect_uri
    );

    Ok(Json(serde_json::json!({ "url": auth_url })))
}

async fn spotify_auth_callback(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<SpotifyAuthRequestDto>,
) -> Result<Json<AuthResponseDto>, AppError> {
    let access_token = state.security.create_token(&payload.code)?;

    Ok(Json(AuthResponseDto {
        access_token,
        refresh_token: "simulated-refresh-token".to_string(),
        expires_in: 3600,
    }))
}

async fn youtube_auth_url(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, AppError> {
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&response_type=code&redirect_uri={}&scope=https://www.googleapis.com/auth/youtube.readonly",
        state.config.youtube_client_id,
        state.config.youtube_redirect_uri
    );

    Ok(Json(serde_json::json!({ "url": auth_url })))
}

async fn youtube_auth_callback(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<YouTubeAuthRequestDto>,
) -> Result<Json<AuthResponseDto>, AppError> {
    let access_token = state.security.create_token(&payload.code)?;

    Ok(Json(AuthResponseDto {
        access_token,
        refresh_token: "simulated-refresh-token".to_string(),
        expires_in: 3600,
    }))
}
