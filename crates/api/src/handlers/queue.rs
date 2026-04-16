use crate::{AppError, AppState};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use domain::{RequestSongRequest, VoteRequest};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct QueueResponse {
    pub items: Vec<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct VoteResponse {
    pub votes: i32,
}

pub async fn request_song(
    State(_state): State<AppState>,
    Json(_request): Json<RequestSongRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), AppError> {
    // TODO: Implement in Phase 3+
    // - Validate room exists
    // - Search provider
    // - Add to queue
    Err(AppError::InternalError("Not implemented yet".to_string()))
}

pub async fn get_queue(
    State(_state): State<AppState>,
    Path(_room_id): Path<String>,
) -> Result<Json<QueueResponse>, AppError> {
    // TODO: Implement in Phase 3+
    // - Load from database or cache
    // - Sort by priority
    Err(AppError::InternalError("Not implemented yet".to_string()))
}

pub async fn vote_song(
    State(_state): State<AppState>,
    Json(_request): Json<VoteRequest>,
) -> Result<Json<VoteResponse>, AppError> {
    // TODO: Implement in Phase 3+
    // - Validate user hasn't voted
    // - Insert vote
    // - Increment votes counter
    Err(AppError::InternalError("Not implemented yet".to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_queue_handlers_exist() {
        // Handlers will be fully tested in integration tests once DB is connected
    }
}
