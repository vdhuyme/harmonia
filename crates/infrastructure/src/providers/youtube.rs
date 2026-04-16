//! YouTube Music provider implementation
//! This module mirrors the structure of the Spotify provider and will be
//! fleshed out in Phase 6.  For now it contains the basic scaffolding,
//! data structures and trait implementation needed to compile the project
//! and to allow later incremental development.

use async_trait::async_trait;
use domain::traits::{MusicProvider, ProviderTrack};
use domain::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// YouTube track representation (partial)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YouTubeTrack {
    pub id: String,
    pub title: String,
    pub channel_title: String,
    pub video_id: String,
    pub url: String,
    pub duration_ms: u64,
}

/// YouTube search response (partial)
#[derive(Debug, Deserialize)]
pub struct YouTubeSearchResponse {
    pub items: Vec<YouTubeItem>,
}

#[derive(Debug, Deserialize)]
pub struct YouTubeItem {
    pub id: YouTubeVideoId,
    pub snippet: YouTubeSnippet,
    pub contentDetails: YouTubeContentDetails,
}

#[derive(Debug, Deserialize)]
pub struct YouTubeVideoId {
    #[serde(rename = "videoId")]
    pub video_id: String,
}

#[derive(Debug, Deserialize)]
pub struct YouTubeSnippet {
    pub title: String,
    pub channelTitle: String,
    pub thumbnails: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct YouTubeContentDetails {
    pub duration: String,
}

/// Provider struct – holds a HTTP client and an API key
pub struct YouTubeProvider {
    http_client: Arc<Client>,
    api_key: String,
    base_url: String,
}

impl YouTubeProvider {
    /// Create a new provider instance.
    /// * `api_key` – YouTube Data API v3 key.
    pub fn new(api_key: String) -> Self {
        Self {
            http_client: Arc::new(Client::new()),
            api_key,
            base_url: "https://www.googleapis.com/youtube/v3".to_string(),
        }
    }

    /// Helper to build the Authorization header if required.
    fn auth_header(&self) -> String {
        // YouTube API uses API‑key query param; header kept for parity with
        // other providers and future OAuth support.
        format!("Bearer {}", self.api_key)
    }
}

#[async_trait]
impl MusicProvider for YouTubeProvider {
    /// Search for tracks (videos) on YouTube.
    async fn search(&self, query: &str) -> Result<Vec<ProviderTrack>, AppError> {
        let url = format!("{}/search", self.base_url);
        let response = self
            .http_client
            .get(&url)
            .query(&[
                ("part", "snippet"),
                ("q", query),
                ("type", "video"),
                ("maxResults", "10"),
                ("key", &self.api_key),
            ])
            .send()
            .await
            .map_err(|e| AppError::ProviderError(format!("YouTube API error: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::ProviderError(format!(
                "YouTube API error: {}",
                response.status()
            )));
        }

        let results: YouTubeSearchResponse = response
            .json()
            .await
            .map_err(|e| {
                AppError::ProviderError(format!("Failed to parse YouTube search results: {}", e))
            })?;

        // Map YouTube items to generic ProviderTrack
        let tracks = results
            .items
            .into_iter()
            .map(|item| ProviderTrack {
                id: item.id.video_id,
                title: item.snippet.title,
                artist: item.snippet.channelTitle,
                duration_ms: 0, // Duration parsing can be added later
            })
            .collect();

        Ok(tracks)
    }

    /// Play a track – placeholder implementation.
    async fn play(&self, _device_id: &str, _track_id: &str) -> Result<(), AppError> {
        // YouTube playback is handled client‑side (web player, TV, etc.).
        // The function is provided to satisfy the MusicProvider trait and
        // will be expanded in Phase 6.
        Ok(())
    }

    /// Pause playback – placeholder.
    async fn pause(&self, _device_id: &str) -> Result<(), AppError> {
        Ok(())
    }

    /// Skip track – placeholder.
    async fn skip(&self, _device_id: &str) -> Result<(), AppError> {
        Ok(())
    }

    /// Get current playback – placeholder returning None.
    async fn get_current_playback(
        &self,
        _device_id: &str,
    ) -> Result<Option<ProviderTrack>, AppError> {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_youtube_provider_creation() {
        let prov = YouTubeProvider::new("dummy_key".to_string());
        assert_eq!(prov.api_key, "dummy_key");
    }

    #[test]
    fn test_auth_header_format() {
        let prov = YouTubeProvider::new("key123".to_string());
        assert_eq!(prov.auth_header(), "Bearer key123");
    }
}