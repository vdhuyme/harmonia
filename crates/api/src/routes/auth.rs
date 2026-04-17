use crate::dto::{
    AuthResponseDto, AuthUrlResponseDto, SpotifyAuthRequestDto,
    YouTubeAuthRequestDto,
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

#[utoipa::path(
    get,
    path = "/auth/spotify/url",
    tag = "Auth",
    responses(
        (status = 200, description = "Spotify OAuth URL", body = AuthUrlResponseDto)
    )
)]
pub async fn spotify_auth_url(
    State(state): State<AppState>,
) -> Result<Json<AuthUrlResponseDto>, AppError> {
    let auth_url = format!(
        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&scope=user-read-private%20user-read-email%20streaming%20user-modify-playback-state",
        state.config.spotify_client_id,
        state.config.spotify_redirect_uri
    );

    Ok(Json(AuthUrlResponseDto { url: auth_url }))
}

#[utoipa::path(
    post,
    path = "/auth/spotify/callback",
    tag = "Auth",
    request_body = SpotifyAuthRequestDto,
    responses(
        (status = 200, description = "Spotify auth callback result", body = AuthResponseDto)
    )
)]
pub async fn spotify_auth_callback(
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

#[utoipa::path(
    get,
    path = "/auth/youtube/url",
    tag = "Auth",
    responses(
        (status = 200, description = "YouTube OAuth URL", body = AuthUrlResponseDto)
    )
)]
pub async fn youtube_auth_url(
    State(state): State<AppState>,
) -> Result<Json<AuthUrlResponseDto>, AppError> {
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&response_type=code&redirect_uri={}&scope=https://www.googleapis.com/auth/youtube.readonly",
        state.config.youtube_client_id,
        state.config.youtube_redirect_uri
    );

    Ok(Json(AuthUrlResponseDto { url: auth_url }))
}

#[utoipa::path(
    post,
    path = "/auth/youtube/callback",
    tag = "Auth",
    request_body = YouTubeAuthRequestDto,
    responses(
        (status = 200, description = "YouTube auth callback result", body = AuthResponseDto)
    )
)]
pub async fn youtube_auth_callback(
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
