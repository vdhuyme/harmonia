use crate::routes;
use crate::state::AppState;
use axum::Router;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::health::health,
        crate::routes::queue::request_song,
        crate::routes::queue::get_queue,
        crate::routes::queue::vote_song,
        crate::routes::auth::spotify_auth_url,
        crate::routes::auth::spotify_auth_callback,
        crate::routes::auth::youtube_auth_url,
        crate::routes::auth::youtube_auth_callback,
        crate::routes::playback::play_music
    ),
    components(
        schemas(
            crate::dto::SongRequestDto,
            crate::dto::SongResponseDto,
            crate::dto::RoomPathDto,
            crate::dto::VotePathDto,
            crate::dto::VoteRequestDto,
            crate::dto::QueueResponseDto,
            crate::dto::QueueItemDto,
            crate::dto::SpotifyAuthRequestDto,
            crate::dto::YouTubeAuthRequestDto,
            crate::dto::AuthResponseDto,
            crate::dto::AuthUrlResponseDto,
            crate::dto::MessageResponseDto,
            crate::dto::PlayRequestDto,
            crate::dto::PlaybackTrackDto,
            crate::dto::PlaybackResponseDto
        )
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Queue", description = "Queue management endpoints"),
        (name = "Auth", description = "Authentication endpoints"),
        (name = "Playback", description = "Playback control endpoints")
    )
)]
pub struct ApiDoc;

pub fn openapi_spec() -> utoipa::openapi::OpenApi {
    ApiDoc::openapi()
}

pub fn build_app(state: AppState) -> Router {
    Router::new()
        .merge(
            SwaggerUi::new("/docs").url("/docs/openapi.json", openapi_spec()),
        )
        .merge(routes::router())
        .layer(CorsLayer::permissive())
        .with_state(state)
}
