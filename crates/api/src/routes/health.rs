use crate::state::AppState;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

#[utoipa::path(
    get,
    path = "/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = String)
    )
)]
pub async fn health() -> impl IntoResponse {
    "OK"
}
