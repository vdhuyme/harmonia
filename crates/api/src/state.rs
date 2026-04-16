use axum::extract::FromRef;

#[derive(Clone)]
pub struct AppState {
    /// JWT secret key for token encoding/decoding
    pub jwt_secret: String,
    /// Redis client for pub/sub and key‑value store
    pub redis: redis::Client,
    // Will be populated with actual services in Phase 4+
    // pub db: Arc<Database>,
    // pub queue_engine: Arc<QueueEngine>,
}

impl AppState {
    /// Create a new AppState from environment
    pub fn from_env() -> Self {
        let jwt_secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "default-dev-secret-change-in-production".to_string());

Self {
    jwt_secret,
    // Connect to Redis using the default URL (override with REDIS_URL env var)
    redis: redis::Client::open(std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string()))
        .expect("Failed to create Redis client"),
}
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            jwt_secret: "default-dev-secret-change-in-production".to_string(),
        }
    }
}
