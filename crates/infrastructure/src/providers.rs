use crate::security::SecurityService;
use async_trait::async_trait;
use domain::error::{DomainError, DomainResult};
use domain::models::Track;
use reqwest;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Generic Music Provider trait
#[async_trait]
pub trait MusicProvider: Send + Sync {
    async fn search_tracks(&self, query: &str) -> DomainResult<Vec<Track>>;
    async fn get_track(&self, track_id: &str) -> DomainResult<Track>;
}

/// Spotify Provider Adapter
pub struct SpotifyProvider {
    client: reqwest::Client,
    security: Arc<SecurityService>,
}

impl SpotifyProvider {
    pub fn new(security: Arc<SecurityService>) -> Self {
        Self {
            client: reqwest::Client::new(),
            security,
        }
    }
}

#[async_trait]
impl MusicProvider for SpotifyProvider {
    async fn search_tracks(&self, query: &str) -> DomainResult<Vec<Track>> {
        // Placeholder implementation - In real implementation, call Spotify API
        // Use self.security to decrypt stored tokens if needed
        Ok(vec![])
    }

    async fn get_track(&self, track_id: &str) -> DomainResult<Track> {
        // Placeholder - Call Spotify API
        Err(DomainError::NotFound("Track not found".to_string()))
    }
}

/// YouTube Provider Adapter
pub struct YouTubeProvider {
    client: reqwest::Client,
    api_key: String,
}

impl YouTubeProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
        }
    }
}

#[async_trait]
impl MusicProvider for YouTubeProvider {
    async fn search_tracks(&self, query: &str) -> DomainResult<Vec<Track>> {
        // Placeholder - Call YouTube Data API
        // URL: https://www.googleapis.com/youtube/v3/search
        Ok(vec![])
    }

    async fn get_track(&self, track_id: &str) -> DomainResult<Track> {
        // Placeholder - Call YouTube API
        Err(DomainError::NotFound("Track not found".to_string()))
    }
}

/// Provider resolver to switch between providers
pub struct ProviderResolver {
    spotify: Arc<dyn MusicProvider>,
    youtube: Arc<dyn MusicProvider>,
}

impl ProviderResolver {
    pub fn new(
        spotify: Arc<dyn MusicProvider>,
        youtube: Arc<dyn MusicProvider>,
    ) -> Self {
        Self { spotify, youtube }
    }

    pub fn get_provider(&self, name: &str) -> Option<Arc<dyn MusicProvider>> {
        match name.to_lowercase().as_str() {
            "spotify" => Some(self.spotify.clone()),
            "youtube" => Some(self.youtube.clone()),
            _ => None,
        }
    }
}
