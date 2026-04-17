use crate::dto::{
    QueueItemDto, QueueResponseDto, RoomPathDto, SongRequestDto,
    SongResponseDto, VotePathDto, VoteRequestDto,
};
use crate::error::AppError;
use crate::extractors::{ValidatedJson, ValidatedPath};
use crate::state::AppState;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/songs/request", post(request_song))
        .route("/queue/{room_id}", get(get_queue))
        .route("/queue/{room_id}/items/{item_id}/vote", post(vote_song))
}

async fn request_song(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<SongRequestDto>,
) -> Result<Json<SongResponseDto>, AppError> {
    let room_id = payload.room_id;
    let user_id = payload.user_id;
    let track = payload.into_track();

    let id = state
        .queue_service
        .add_song(room_id, user_id, track)
        .await?;

    Ok(Json(SongResponseDto {
        id: id.to_string(),
        message: "Song added to queue".to_string(),
    }))
}

async fn get_queue(
    State(state): State<AppState>,
    ValidatedPath(path): ValidatedPath<RoomPathDto>,
) -> Result<Json<QueueResponseDto>, AppError> {
    let items = state.queue_service.get_sorted_queue(path.room_id).await?;

    let items = items
        .into_iter()
        .map(|item| QueueItemDto {
            id: item.id.to_string(),
            title: item.track.title,
            artist: item.track.artist,
            votes: item.votes,
            added_by: item.added_by.to_string(),
        })
        .collect();

    Ok(Json(QueueResponseDto { items }))
}

async fn vote_song(
    State(state): State<AppState>,
    ValidatedPath(path): ValidatedPath<VotePathDto>,
    ValidatedJson(payload): ValidatedJson<VoteRequestDto>,
) -> Result<Json<serde_json::Value>, AppError> {
    state
        .queue_service
        .vote_song(path.room_id, payload.user_id, path.item_id, payload.value)
        .await?;

    Ok(Json(serde_json::json!({ "message": "Vote recorded" })))
}
