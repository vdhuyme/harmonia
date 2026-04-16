use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use base64::{engine::general_purpose, Engine as _};
use domain::models::{MusicProvider, Track};
use infrastructure::{
    ProviderResolver, QueueEngine, RedisClient, SecurityService,
    SpotifyProvider, SqlRepository, YouTubeProvider,
};
use sea_orm::prelude::Uuid;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    queue_engine: Arc<QueueEngine>,
    security: Arc<SecurityService>,
    provider_resolver: Arc<ProviderResolver>,
}

async fn health() -> impl IntoResponse {
    "OK"
}

#[derive(Deserialize)]
struct SongRequest {
    room_id: String,
    track_id: String,
    provider: String,
    title: String,
    artist: String,
    uri: String,
    user_id: String,
    album: Option<String>,
    artwork_url: Option<String>,
    duration_ms: Option<u32>,
}

#[derive(Serialize)]
struct SongResponse {
    id: String,
    message: String,
}

async fn request_song(
    State(state): State<AppState>,
    Json(payload): Json<SongRequest>,
) -> Result<impl IntoResponse, AppError> {
    let room_id = Uuid::parse_str(&payload.room_id)
        .map_err(|_| AppError("Invalid room ID".to_string()))?;
    let user_id = Uuid::parse_str(&payload.user_id)
        .map_err(|_| AppError("Invalid user ID".to_string()))?;

    let provider = match payload.provider.to_lowercase().as_str() {
        "spotify" => MusicProvider::Spotify,
        "youtube" => MusicProvider::YouTube,
        _ => return Err(AppError("Invalid provider".to_string())),
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
    let room_uuid = Uuid::parse_str(&room_id)
        .map_err(|_| AppError("Invalid room ID".to_string()))?;

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

#[derive(Deserialize)]
struct VoteRequest {
    user_id: String,
    value: i8,
}

async fn vote_song(
    State(state): State<AppState>,
    Path((room_id, item_id)): Path<(String, String)>,
    Json(payload): Json<VoteRequest>,
) -> Result<impl IntoResponse, AppError> {
    let room_uuid = Uuid::parse_str(&room_id)
        .map_err(|_| AppError("Invalid room ID".to_string()))?;
    let user_uuid = Uuid::parse_str(&payload.user_id)
        .map_err(|_| AppError("Invalid user ID".to_string()))?;
    let item_uuid = Uuid::parse_str(&item_id)
        .map_err(|_| AppError("Invalid item ID".to_string()))?;

    state
        .queue_engine
        .vote_song(room_uuid, user_uuid, item_uuid, payload.value)
        .await
        .map_err(|e| AppError(e.to_string()))?;

    Ok(Json(serde_json::json!({ "message": "Vote recorded" })))
}

// Authentication endpoints
#[derive(Deserialize)]
struct SpotifyAuthRequest {
    code: String,
    state: String,
}

#[derive(Deserialize)]
struct YouTubeAuthRequest {
    code: String,
    state: String,
}

#[derive(Serialize)]
struct AuthResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
}

async fn spotify_auth_url(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    // In a real implementation, generate a proper Spotify auth URL
    let auth_url = format!(
        "https://accounts.spotify.com/authorize?client_id={}&response_type=code&redirect_uri={}&scope=user-read-private%20user-read-email%20streaming%20user-modify-playback-state",
        "your-spotify-client-id", // Should come from config
        "http://localhost:3000/auth/spotify/callback"
    );

    Ok(Json(serde_json::json!({ "url": auth_url })))
}

async fn spotify_auth_callback(
    State(state): State<AppState>,
    Json(payload): Json<SpotifyAuthRequest>,
) -> Result<impl IntoResponse, AppError> {
    // In a real implementation, exchange code for tokens with Spotify
    // For now, we'll simulate successful authentication
    let access_token = state.security.create_token(&payload.code)?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token: "simulated-refresh-token".to_string(),
        expires_in: 3600,
    }))
}

async fn youtube_auth_url(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    // In a real implementation, generate a proper YouTube auth URL
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&response_type=code&redirect_uri={}&scope=https://www.googleapis.com/auth/youtube.readonly",
        "your-youtube-client-id", // Should come from config
        "http://localhost:3000/auth/youtube/callback"
    );

    Ok(Json(serde_json::json!({ "url": auth_url })))
}

async fn youtube_auth_callback(
    State(state): State<AppState>,
    Json(payload): Json<YouTubeAuthRequest>,
) -> Result<impl IntoResponse, AppError> {
    // In a real implementation, exchange code for tokens with YouTube
    // For now, we'll simulate successful authentication
    let access_token = state.security.create_token(&payload.code)?;

    Ok(Json(AuthResponse {
        access_token,
        refresh_token: "simulated-refresh-token".to_string(),
        expires_in: 3600,
    }))
}

// Provider control endpoints (would typically be in a workers service)
#[derive(Deserialize)]
struct PlayRequest {
    room_id: String,
    user_id: String,
}

async fn play_music(
    State(state): State<AppState>,
    Json(payload): Json<PlayRequest>,
) -> Result<impl IntoResponse, AppError> {
    let room_uuid = Uuid::parse_str(&payload.room_id)
        .map_err(|_| AppError("Invalid room ID".to_string()))?;
    let _user_uuid = Uuid::parse_str(&payload.user_id)
        .map_err(|_| AppError("Invalid user ID".to_string()))?;

    // Get the next song to play from the queue
    let queue_items = state
        .queue_engine
        .get_sorted_queue(room_uuid)
        .await
        .map_err(|e| AppError(e.to_string()))?;

    if queue_items.is_empty() {
        return Err(AppError("Queue is empty".to_string()));
    }

    let next_item = &queue_items[0];

    // In a real implementation, we would:
    // 1. Get the user's provider credentials from the database
    // 2. Decrypt the tokens using the security service
    // 3. Use the appropriate provider to play the track

    // For now, we'll just return success
    Ok(Json(serde_json::json!({
        "message": "Playback started",
        "track": {
            "id": next_item.id.to_string(),
            "title": next_item.track.title.clone(),
            "artist": next_item.track.artist.clone()
        }
    })))
}

#[derive(Debug)]
struct AppError(String);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (axum::http::StatusCode::INTERNAL_SERVER_ERROR, self.0).into_response()
    }
}

impl From<domain::error::DomainError> for AppError {
    fn from(e: domain::error::DomainError) -> Self {
        AppError(e.to_string())
    }
}

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
        "your-jwt-secret".to_string(), // Should come from config/environment
        vec![0u8; 32], // 256-bit encryption key (should come from secure source)
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
