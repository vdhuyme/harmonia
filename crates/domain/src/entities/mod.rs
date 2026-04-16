use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QueueStatus {
    Pending,
    Playing,
    Skipped,
    Done,
}

impl QueueStatus {
    pub fn as_str(&self) -> &str {
        match self {
            QueueStatus::Pending => "pending",
            QueueStatus::Playing => "playing",
            QueueStatus::Skipped => "skipped",
            QueueStatus::Done => "done",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "pending" => Ok(QueueStatus::Pending),
            "playing" => Ok(QueueStatus::Playing),
            "skipped" => Ok(QueueStatus::Skipped),
            "done" => Ok(QueueStatus::Done),
            _ => Err(format!("Invalid status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProviderType {
    Spotify,
    YouTube,
}

impl ProviderType {
    pub fn as_str(&self) -> &str {
        match self {
            ProviderType::Spotify => "spotify",
            ProviderType::YouTube => "youtube",
        }
    }

    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "spotify" => Ok(ProviderType::Spotify),
            "youtube" => Ok(ProviderType::YouTube),
            _ => Err(format!("Invalid provider: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub id: String,
    pub room_id: String,
    pub provider: ProviderType,
    pub track_id: String,
    pub title: String,
    pub artist: String,
    pub priority: i32,
    pub votes: i32,
    pub status: QueueStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderAccount {
    pub id: String,
    pub provider: ProviderType,
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomMapping {
    pub id: String,
    pub room_id: String,
    pub provider_account_id: String,
    pub device_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub id: String,
    pub queue_item_id: String,
    pub user_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// Request/Response DTOs
#[derive(Debug, Deserialize)]
pub struct RequestSongRequest {
    pub query: String,
    pub room_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub provider: String,
    pub duration_ms: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct VoteRequest {
    pub queue_item_id: String,
    pub user_id: String,
}

#[derive(Debug, Deserialize)]
pub struct ConnectProviderRequest {
    pub provider: String,
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Deserialize)]
pub struct MapRoomRequest {
    pub room_id: String,
    pub provider_account_id: String,
    pub device_id: String,
}
