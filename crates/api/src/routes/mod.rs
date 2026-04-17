pub mod auth;
pub mod health;
pub mod playback;
pub mod queue;

use crate::state::AppState;
use axum::Router;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(health::router())
        .merge(queue::router())
        .merge(auth::router())
        .merge(playback::router())
}
