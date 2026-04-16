//! API Gateway - Axum HTTP server with REST and WebSocket support

pub mod crypto;
pub mod error;
pub mod handlers;
pub mod jwt;
pub mod middleware;
pub mod state;

pub use error::AppError;
pub use state::AppState;

use crate::handlers::websocket;
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

/// Build the API router
pub fn router(state: AppState) -> Router<(AppState, Arc<jwt::JwtHandler>)> {
    let jwt_state = Arc::new(jwt::JwtHandler::new(state.jwt_secret.as_bytes()));

    Router::new()
        .route("/health", get(handlers::health::health))
        // Auth endpoints (public)
        .route("/auth/login", post(handlers::auth::login))
        .route("/auth/register", post(handlers::auth::register))
        .route("/auth/spotify/url", get(handlers::oauth::spotify_auth_url))
        .route(
            "/auth/spotify/callback",
            post(handlers::oauth::spotify_callback),
        )
        // Queue endpoints (protected)
        .route("/songs/request", post(handlers::queue::request_song))
        .route("/queue/:room_id", get(handlers::queue::get_queue))
        .route("/queue/:room_id/vote", post(handlers::queue::vote_song))
        // Admin endpoints (protected)
        .route(
            "/admin/provider/connect",
            post(handlers::admin::connect_provider),
        )
        .route("/admin/room/map", post(handlers::admin::map_room))
        .layer(axum::middleware::from_fn(middleware_rate_limit))
        .nest("/ws", websocket::router())
        .with_state((state, jwt_state))
}

/// Simple rate limiting middleware
async fn middleware_rate_limit(
    req: axum::http::Request<axum::body::Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    // TODO: Implement rate limiting using Redis
    // For now, just pass through
    next.run(req).await
}
