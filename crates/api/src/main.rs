mod app;
mod config;
mod dto;
mod error;
mod extractors;
mod routes;
mod state;

use crate::app::build_app;
use crate::config::Config;
use crate::state::build_app_state;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config::from_env();
    let state = build_app_state(config.clone())
        .await
        .expect("failed to initialize app state");
    let app = build_app(state);

    let listener = tokio::net::TcpListener::bind(config.bind_addr())
        .await
        .expect("failed to bind API listener");

    tracing::info!("API listening on {}", config.bind_addr());
    axum::serve(listener, app)
        .await
        .expect("failed to start API server");
}
