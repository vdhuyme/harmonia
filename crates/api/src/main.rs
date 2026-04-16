use api::AppState;
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    tracing::info!("🎵 Music Queue Platform API Server");

    // In production, load from environment
    let api_host =
        std::env::var("API_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let api_port = std::env::var("API_PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("API_PORT must be a valid port number");

    let addr = format!("{}:{}", api_host, api_port);
    tracing::info!("Starting server on http://{}", addr);

    // Create app state from environment
    let state = AppState::from_env();
    tracing::info!(
        "JWT secret configured: {}",
        if state.jwt_secret.len() > 20 {
            "custom"
        } else {
            "default (dev mode)"
        }
    );

    // Build router
    let app = api::router(state);

    // Run server
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.expect("Server failed");
}
