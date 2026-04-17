use crate::dto::{PlayRequestDto, PlaybackResponseDto, PlaybackTrackDto};
use crate::error::AppError;
use crate::extractors::ValidatedJson;
use crate::state::AppState;
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};

pub fn router() -> Router<AppState> {
    Router::new().route("/play", post(play_music))
}

#[utoipa::path(
    post,
    path = "/play",
    tag = "Playback",
    request_body = PlayRequestDto,
    responses(
        (status = 200, description = "Playback started", body = PlaybackResponseDto)
    )
)]
pub async fn play_music(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<PlayRequestDto>,
) -> Result<Json<PlaybackResponseDto>, AppError> {
    let _requester = payload.user_id;
    let decision = state.playback_service.next_track(payload.room_id).await?;

    Ok(Json(PlaybackResponseDto {
        message: "Playback started".to_string(),
        track: PlaybackTrackDto {
            id: decision.item.id.to_string(),
            title: decision.item.track.title,
            artist: decision.item.track.artist,
        },
    }))
}
