use crate::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderTrack {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub duration_ms: i32,
}

#[async_trait]
pub trait MusicProvider: Send + Sync {
    async fn search(&self, query: &str) -> Result<Vec<ProviderTrack>>;
    async fn play(&self, device_id: &str, track_id: &str) -> Result<()>;
    async fn pause(&self, device_id: &str) -> Result<()>;
    async fn skip(&self, device_id: &str) -> Result<()>;
    async fn get_current_playback(
        &self,
        device_id: &str,
    ) -> Result<Option<ProviderTrack>>;
}
