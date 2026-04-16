use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use domain::models::{MusicProvider, Track};
use infrastructure::{
    ProviderResolver, QueueEngine, RedisClient, SecurityService,
    SpotifyProvider, SqlRepository, YouTubeProvider,
};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use validator::Validate;

// ============================================================================
// ValidatedJson Extractor - Validates JSON input before handler execution
// ============================================================================

use axum::extract::FromRequest;
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde::de::DeserializeOwned;

/// Extractor that validates JSON input using validator derive
pub struct ValidatedJson<T>(pub T)
where
    T: Validate + DeserializeOwned + Send + 'static;

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: Validate + DeserializeOwned + Send + 'static,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(
        req: Request<Body>,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        // Use axum's Json extractor to parse the body
        let payload: T = axum::extract::Json::from_request(req, state)
            .await
            .map_err(|e| {
                (StatusCode::BAD_REQUEST, format!("Invalid JSON: {}", e))
            })?
            .0;

        payload
            .validate()
            .map(|_| ValidatedJson(payload))
            .map_err(|errors| {
                (StatusCode::BAD_REQUEST, format_validation_errors(&errors))
            })
    }
}

// Helper to convert validation errors to a simple string
fn format_validation_errors(errors: &validator::ValidationErrors) -> String {
    errors
        .field_errors()
        .into_iter()
        .map(|(field, err_vec)| {
            let msgs: Vec<String> = err_vec
                .iter()
                .map(|e| e.message.as_deref().unwrap_or("invalid").to_string())
                .collect();
            format!("{}: {}", field, msgs.join(", "))
        })
        .collect::<Vec<_>>()
        .join("; ")
}

// ============================================================================
// App State
// ============================================================================

#[derive(Clone)]
struct AppState {
    queue_engine: Arc<QueueEngine>,
    security: Arc<SecurityService>,
    #[allow(dead_code)]
    provider_resolver: Arc<ProviderResolver>,
}

// ============================================================================
// Health Check
// ============================================================================

async fn health() -> impl IntoResponse {
    "OK"
}

// ============================================================================
// Request DTOs with Validation
// ============================================================================

/// Request to add a song to the queue
#[derive(Deserialize, Serialize, Validate)]
struct SongRequest {
    #[validate(length(min = 1, max = 36))]
    room_id: String,

    #[validate(length(min = 1, max = 256))]
    track_id: String,

    #[validate(length(min = 1, max = 50))]
    provider: String,

    #[validate(length(min = 1, max = 500))]
    title: String,

    #[validate(length(min = 1, max = 500))]
    artist: String,

    #[validate(length(min = 1, max = 2048))]
    uri: String,

    #[validate(length(min = 1, max = 36))]
    user_id: String,

    #[validate(length(max = 500))]
    album: Option<String>,

    #[validate(length(max = 2048))]
    artwork_url: Option<String>,

    #[validate(range(min = 0, max = 86400000))]
    duration_ms: Option<u32>,
}

#[derive(Serialize)]
struct SongResponse {
    id: String,
    message: String,
}

async fn request_song(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<SongRequest>,
) -> Result<impl IntoResponse, AppError> {
    let room_id = Uuid::parse_str(&payload.room_id)
        .map_err(|_| AppError("Invalid room ID format".to_string()))?;
    let user_id = Uuid::parse_str(&payload.user_id)
        .map_err(|_| AppError("Invalid user ID format".to_string()))?;

    let provider = match payload.provider.to_lowercase().as_str() {
        "spotify" => MusicProvider::Spotify,
        "youtube" => MusicProvider::YouTube,
        _ => {
            return Err(AppError(
                "Invalid provider. Must be 'spotify' or 'youtube'".to_string(),
            ))
        }
    };

    let track = Track {
        id: payload.track_id,
        provider,
        title: payload.title,
        artist: payload.artist,
        album: payload.album,
        duration_ms: payload.duration_ms.unwrap_or(0),
        uri: payload.uri,
        artwork_url: payload.artwork_url,
    };

    let id = state
        .queue_engine
        .add_song(room_id, user_id, track)
        .await
        .map_err(|e| AppError(e.to_string()))?;

    Ok(Json(SongResponse {
        id: id.to_string(),
        message: "Song added to queue".to_string(),
    }))
}

// ============================================================================
// Queue Endpoints
// ============================================================================

#[derive(Serialize)]
struct QueueResponse {
    items: Vec<QueueItemDto>,
}

#[derive(Serialize)]
struct QueueItemDto {
    id: String,
    title: String,
    artist: String,
    votes: i32,
    added_by: String,
}

async fn get_queue(
    State(state): State<AppState>,
    Path(room_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
    // Validate room_id format before parsing to UUID
    if room_id.trim().is_empty() {
        return Err(AppError("Room ID cannot be empty".to_string()));
    }

    let room_uuid = Uuid::parse_str(&room_id).map_err(|_| {
        AppError("Invalid room ID format. Must be a valid UUID".to_string())
    })?;

    let items = state
        .queue_engine
        .get_sorted_queue(room_uuid)
        .await
        .map_err(|e| AppError(e.to_string()))?;

    let dtos = items
        .into_iter()
        .map(|item| QueueItemDto {
            id: item.id.to_string(),
            title: item.track.title,
            artist: item.track.artist,
            votes: item.votes,
            added_by: item.added_by.to_string(),
        })
        .collect();

    Ok(Json(QueueResponse { items: dtos }))
}

/// Vote request
#[derive(Deserialize, Validate)]
struct VoteRequest {
    #[validate(length(min = 1, max = 36))]
    user_id: String,

    #[validate(range(min = -10, max = 10))]
    value: i8,
}

async fn vote_song(
    State(state): State<AppState>,
    Path((room_id, item_id)): Path<(String, String)>,
    ValidatedJson(payload): ValidatedJson<VoteRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Validate path parameters
    if room_id.trim().is_empty() {
        return Err(AppError("Room ID cannot be empty".to_string()));
    }
    if item_id.trim().is_empty() {
        return Err(AppError("Item ID cannot be empty".to_string()));
    }

    let room_uuid = Uuid::parse_str(&room_id)
        .map_err(|_| AppError("Invalid room ID format".to_string()))?;
    let user_uuid = Uuid::parse_str(&payload.user_id)
        .map_err(|_| AppError("Invalid user ID format".to_string()))?;
    let item_uuid = Uuid::parse_str(&item_id)
        .map_err(|_| AppError("Invalid item ID format".to_string()))?;

    state
        .queue_engine
        .vote_song(room_uuid, user_uuid, item_uuid, payload.value)
        .await
        .map_err(|e| AppError(e.to_string()))?;

    Ok(Json(serde_json::json!({ "message": "Vote recorded" })))
}

// ============================================================================
// Authentication Endpoints
// ============================================================================

#[derive(Deserialize, Validate)]
struct SpotifyAuthRequest {
    #[validate(length(min = 1, max = 2048))]
    code: String,

    #[allow(dead_code)]
    #[validate(length(max = 256))]
    state: String,
}

#[derive(Deserialize, Validate)]
struct YouTubeAuthRequest {
    #[validate(length(min = 1, max = 2048))]
    code: String,

    #[allow(dead_code)]
    #[validate(length(max = 256))]
    state: String,
}

#[derive(Serialize)]
struct AuthResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}

async fn spotify_auth_url(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let auth_url = format!(
        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&scope=user-read-private%20user-read-email%20streaming%20user-modify-playback-state",
        "your-spotify-client-id",
        "http://localhost:3000/auth/spotify/callback"
    );

    Ok(Json(serde_json::json!({ "url": auth_url })))
}

async fn spotify_auth_callback(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<SpotifyAuthRequest>,
) -> Result<impl IntoResponse, AppError> {
    let access_token = state.security.create_token(&payload.code)?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token: "simulated-refresh-token".to_string(),
        expires_in: 3600,
    }))
}

async fn youtube_auth_url(
    State(_state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&response_type=code&redirect_uri={}&scope=https://www.googleapis.com/auth/youtube.readonly",
        "your-youtube-client-id",
        "http://localhost:3000/auth/youtube/callback"
    );

    Ok(Json(serde_json::json!({ "url": auth_url })))
}

async fn youtube_auth_callback(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<YouTubeAuthRequest>,
) -> Result<impl IntoResponse, AppError> {
    let access_token = state.security.create_token(&payload.code)?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token: "simulated-refresh-token".to_string(),
        expires_in: 3600,
    }))
}

// ============================================================================
// Playback Control
// ============================================================================

#[derive(Deserialize, Validate)]
struct PlayRequest {
    #[validate(length(min = 1, max = 36))]
    room_id: String,

    #[validate(length(min = 1, max = 36))]
    user_id: String,
}

async fn play_music(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<PlayRequest>,
) -> Result<impl IntoResponse, AppError> {
    let room_uuid = Uuid::parse_str(&payload.room_id)
        .map_err(|_| AppError("Invalid room ID format".to_string()))?;
    let _user_uuid = Uuid::parse_str(&payload.user_id)
        .map_err(|_| AppError("Invalid user ID format".to_string()))?;

    let queue_items = state
        .queue_engine
        .get_sorted_queue(room_uuid)
        .await
        .map_err(|e| AppError(e.to_string()))?;

    if queue_items.is_empty() {
        return Err(AppError("Queue is empty".to_string()));
    }

    let next_item = &queue_items[0];

    Ok(Json(serde_json::json!({
        "message": "Playback started",
        "track": {
            "id": next_item.id.to_string(),
            "title": next_item.track.title.clone(),
            "artist": next_item.track.artist.clone()
        }
    })))
}

// ============================================================================
// Error Handling
// ============================================================================

#[derive(Debug)]
struct AppError(String);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::BAD_REQUEST, self.0).into_response()
    }
}

impl From<domain::error::DomainError> for AppError {
    fn from(e: domain::error::DomainError) -> Self {
        AppError(e.to_string())
    }
}

// ============================================================================
// Main Entry Point
// ============================================================================

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Initialize DB
    let db = sea_orm::Database::connect(
        "postgres://user:password@localhost/harmonia",
    )
    .await
    .expect("Failed to connect to database");

    // Initialize Redis
    let redis = RedisClient::new("redis://127.0.0.1")
        .expect("Failed to connect to Redis");

    // Initialize Security Service
    let security = Arc::new(SecurityService::new(
        "your-jwt-secret".to_string(),
        vec![0u8; 32],
    ));

    // Initialize services
    let repo = Arc::new(SqlRepository::new(db.clone()));
    let redis_arc = Arc::new(redis);
    let queue_engine = Arc::new(QueueEngine::new(repo, redis_arc));

    // Initialize providers
    let spotify_provider = Arc::new(SpotifyProvider::new(security.clone()));
    let youtube_provider =
        Arc::new(YouTubeProvider::new("your-youtube-api-key".to_string()));
    let provider_resolver =
        Arc::new(ProviderResolver::new(spotify_provider, youtube_provider));

    let state = AppState {
        queue_engine,
        security,
        provider_resolver,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/songs/request", post(request_song))
        .route("/queue/:room_id", get(get_queue))
        .route("/queue/:room_id/vote", post(vote_song))
        // Auth endpoints
        .route("/auth/spotify/url", get(spotify_auth_url))
        .route("/auth/spotify/callback", post(spotify_auth_callback))
        .route("/auth/youtube/url", get(youtube_auth_url))
        .route("/auth/youtube/callback", post(youtube_auth_callback))
        // Playback control
        .route("/play", post(play_music))
        .layer(CorsLayer::permissive())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
