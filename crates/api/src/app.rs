use crate::routes;
use crate::state::AppState;
use axum::Router;
use tower_http::cors::CorsLayer;

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .merge(routes::router())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
