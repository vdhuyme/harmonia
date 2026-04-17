mod app;
mod config;
mod dto;
mod error;
mod extractors;
mod routes;
mod state;

use crate::app::build_app;
use crate::app::openapi_spec;
use crate::config::Config;
use crate::state::build_app_state;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if export_openapi_if_requested() {
        return;
    }

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

fn export_openapi_if_requested() -> bool {
    let mut args = std::env::args();
    let _program = args.next();
    let flag = args.next();

    if flag.as_deref() != Some("--export-openapi") {
        return false;
    }

    let output_path = args
        .next()
        .unwrap_or_else(|| "docs/openapi.json".to_string());

    if let Some(parent) = Path::new(&output_path).parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .expect("failed to create OpenAPI output directory");
        }
    }

    let openapi = openapi_spec();
    let payload = serde_json::to_string_pretty(&openapi)
        .expect("failed to serialize OpenAPI document");
    fs::write(&output_path, payload).expect("failed to write OpenAPI document");

    println!("OpenAPI exported to {}", output_path);
    true
}
