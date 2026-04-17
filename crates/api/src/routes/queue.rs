use crate::dto::{
    MessageResponseDto, QueueItemDto, QueueResponseDto, RoomPathDto,
    SongRequestDto, SongResponseDto, VotePathDto, VoteRequestDto,
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

#[utoipa::path(
    post,
    path = "/songs/request",
    tag = "Queue",
    request_body = SongRequestDto,
    responses(
        (status = 200, description = "Song added to queue", body = SongResponseDto)
    )
)]
pub async fn request_song(
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

#[utoipa::path(
    get,
    path = "/queue/{room_id}",
    tag = "Queue",
    params(
        ("room_id" = uuid::Uuid, Path, description = "Room id")
    ),
    responses(
        (status = 200, description = "Sorted queue", body = QueueResponseDto)
    )
)]
pub async fn get_queue(
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

#[utoipa::path(
    post,
    path = "/queue/{room_id}/items/{item_id}/vote",
    tag = "Queue",
    params(
        ("room_id" = uuid::Uuid, Path, description = "Room id"),
        ("item_id" = uuid::Uuid, Path, description = "Queue item id")
    ),
    request_body = VoteRequestDto,
    responses(
        (status = 200, description = "Vote recorded", body = MessageResponseDto)
    )
)]
pub async fn vote_song(
    State(state): State<AppState>,
    ValidatedPath(path): ValidatedPath<VotePathDto>,
    ValidatedJson(payload): ValidatedJson<VoteRequestDto>,
) -> Result<Json<MessageResponseDto>, AppError> {
    state
        .queue_service
        .vote_song(path.room_id, payload.user_id, path.item_id, payload.value)
        .await?;

    Ok(Json(MessageResponseDto {
        message: "Vote recorded".to_string(),
    }))
}
