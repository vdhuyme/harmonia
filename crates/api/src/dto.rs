use domain::models::{MusicProvider, Track};
use serde::{Deserialize, Serialize};
use shared::validation::{validate_non_empty_string, validate_provider};
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct SongRequestDto {
    pub room_id: Uuid,

    #[validate(length(min = 1, max = 256))]
    #[validate(custom(function = "validate_non_empty_string"))]
    pub track_id: String,

    #[validate(custom(function = "validate_provider"))]
    pub provider: String,

    #[validate(length(min = 1, max = 500))]
    #[validate(custom(function = "validate_non_empty_string"))]
    pub title: String,

    #[validate(length(min = 1, max = 500))]
    #[validate(custom(function = "validate_non_empty_string"))]
    pub artist: String,

    #[validate(length(min = 1, max = 2048))]
    pub uri: String,

    pub user_id: Uuid,

    #[validate(length(max = 500))]
    pub album: Option<String>,

    #[validate(length(max = 2048))]
    pub artwork_url: Option<String>,

    #[validate(range(min = 0, max = 86_400_000))]
    pub duration_ms: Option<u32>,
}

impl SongRequestDto {
    pub fn into_track(self) -> Track {
        Track {
            id: self.track_id,
            provider: provider_from_string(&self.provider),
            title: self.title,
            artist: self.artist,
            album: self.album,
            duration_ms: self.duration_ms.unwrap_or(0),
            uri: self.uri,
            artwork_url: self.artwork_url,
        }
    }
}

#[derive(Serialize)]
pub struct SongResponseDto {
    pub id: String,
    pub message: String,
}

#[derive(Deserialize, Validate)]
pub struct RoomPathDto {
    pub room_id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct VotePathDto {
    pub room_id: Uuid,
    pub item_id: Uuid,
}

#[derive(Deserialize, Validate)]
pub struct VoteRequestDto {
    pub user_id: Uuid,

    #[validate(range(min = -10, max = 10))]
    pub value: i8,
}

#[derive(Serialize)]
pub struct QueueResponseDto {
    pub items: Vec<QueueItemDto>,
}

#[derive(Serialize)]
pub struct QueueItemDto {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub votes: i32,
    pub added_by: String,
}

#[derive(Deserialize, Validate)]
pub struct SpotifyAuthRequestDto {
    #[validate(length(min = 1, max = 2048))]
    pub code: String,

    #[validate(length(max = 256))]
    pub state: Option<String>,
}

#[derive(Deserialize, Validate)]
pub struct YouTubeAuthRequestDto {
    #[validate(length(min = 1, max = 2048))]
    pub code: String,

    #[validate(length(max = 256))]
    pub state: Option<String>,
}

#[derive(Serialize)]
pub struct AuthResponseDto {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

#[derive(Deserialize, Validate)]
pub struct PlayRequestDto {
    pub room_id: Uuid,
    pub user_id: Uuid,
}

fn provider_from_string(provider: &str) -> MusicProvider {
    match provider.to_lowercase().as_str() {
        "spotify" => MusicProvider::Spotify,
        "youtube" => MusicProvider::YouTube,
        _ => MusicProvider::Spotify,
    }
}
