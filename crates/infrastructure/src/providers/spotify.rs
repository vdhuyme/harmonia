use async_trait::async_trait;
use domain::traits::{MusicProvider, ProviderTrack};
use domain::AppError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Spotify API response for track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyTrack {
    pub id: String,
    pub name: String,
    pub artists: Vec<SpotifyArtist>,
    pub uri: String,
}

/// Spotify artist info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyArtist {
    pub name: String,
}

/// Spotify search results
#[derive(Debug, Deserialize)]
pub struct SpotifySearchResults {
    pub tracks: TrackResults,
}

#[derive(Debug, Deserialize)]
pub struct TrackResults {
    pub items: Vec<SpotifyTrack>,
}

/// Spotify device for playback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotifyDevice {
    pub id: String,
    pub name: String,
    #[serde(rename = "is_active")]
    pub is_active: bool,
}

/// Spotify devices response
#[derive(Debug, Deserialize)]
pub struct SpotifyDevices {
    pub devices: Vec<SpotifyDevice>,
}

/// Spotify current playback info
#[derive(Debug, Deserialize)]
pub struct SpotifyPlayback {
    pub item: Option<SpotifyTrack>,
    #[serde(rename = "is_playing")]
    pub is_playing: bool,
    pub progress_ms: Option<u32>,
}

/// Spotify provider for music search and playback control
pub struct SpotifyProvider {
    http_client: Arc<Client>,
    access_token: String,
    base_url: String,
}

impl SpotifyProvider {
    /// Create new Spotify provider with access token
    pub fn new(access_token: String) -> Self {
        Self {
            http_client: Arc::new(Client::new()),
            access_token,
            base_url: "https://api.spotify.com/v1".to_string(),
        }
    }

    /// Create authorization header
    fn auth_header(&self) -> String {
        format!("Bearer {}", self.access_token)
    }

    /// Get available devices for playback
    pub async fn get_devices(&self) -> Result<Vec<SpotifyDevice>, AppError> {
        let url = format!("{}/me/player/devices", self.base_url);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| {
                AppError::ProviderError(format!("Spotify API error: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(AppError::ProviderError(format!(
                "Spotify API error: {}",
                response.status()
            )));
        }

        let devices: SpotifyDevices = response.json().await.map_err(|e| {
            AppError::ProviderError(format!("Failed to parse devices: {}", e))
        })?;

        Ok(devices.devices)
    }

    /// Get currently playing track
    pub async fn get_current_playback_internal(
        &self,
    ) -> Result<Option<SpotifyTrack>, AppError> {
        let url = format!("{}/me/player/currently-playing", self.base_url);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .await
            .map_err(|e| {
                AppError::ProviderError(format!("Spotify API error: {}", e))
            })?;

        if response.status().as_u16() == 204 {
            return Ok(None);
        }

        if !response.status().is_success() {
            return Err(AppError::ProviderError(format!(
                "Spotify API error: {}",
                response.status()
            )));
        }

        let playback: SpotifyPlayback = response.json().await.map_err(|e| {
            AppError::ProviderError(format!("Failed to parse playback: {}", e))
        })?;

        Ok(playback.item)
    }
}

#[async_trait]
impl MusicProvider for SpotifyProvider {
    /// Search for tracks on Spotify
    async fn search(
        &self,
        query: &str,
    ) -> Result<Vec<ProviderTrack>, AppError> {
        let url = format!("{}/search", self.base_url);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", self.auth_header())
            .query(&[("q", query), ("type", "track"), ("limit", "10")])
            .send()
            .await
            .map_err(|e| {
                AppError::ProviderError(format!("Spotify API error: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(AppError::ProviderError(format!(
                "Spotify API error: {}",
                response.status()
            )));
        }

        let results: SpotifySearchResults =
            response.json().await.map_err(|e| {
                AppError::ProviderError(format!(
                    "Failed to parse search results: {}",
                    e
                ))
            })?;

        let tracks = results
            .tracks
            .items
            .into_iter()
            .map(|t| ProviderTrack {
                id: t.id,
                title: t.name,
                artist: t
                    .artists
                    .first()
                    .map(|a| a.name.clone())
                    .unwrap_or_default(),
                duration_ms: 0,
            })
            .collect();

        Ok(tracks)
    }

    /// Play a track on a device
    async fn play(
        &self,
        device_id: &str,
        track_id: &str,
    ) -> Result<(), AppError> {
        let url = format!("{}/me/player/play", self.base_url);

        let body = serde_json::json!({
            "uris": [format!("spotify:track:{}", track_id)],
            "device_id": device_id
        });

        let response = self
            .http_client
            .put(&url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json")
            .query(&[("device_id", device_id)])
            .json(&body)
            .send()
            .await
            .map_err(|e| {
                AppError::ProviderError(format!("Spotify API error: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(AppError::ProviderError(format!(
                "Spotify play error: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Pause playback on a device
    async fn pause(&self, device_id: &str) -> Result<(), AppError> {
        let url = format!("{}/me/player/pause", self.base_url);

        let response = self
            .http_client
            .put(&url)
            .header("Authorization", self.auth_header())
            .query(&[("device_id", device_id)])
            .send()
            .await
            .map_err(|e| {
                AppError::ProviderError(format!("Spotify API error: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(AppError::ProviderError(format!(
                "Spotify pause error: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Skip to next track
    async fn skip(&self, device_id: &str) -> Result<(), AppError> {
        let url = format!("{}/me/player/next", self.base_url);

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", self.auth_header())
            .query(&[("device_id", device_id)])
            .send()
            .await
            .map_err(|e| {
                AppError::ProviderError(format!("Spotify API error: {}", e))
            })?;

        if !response.status().is_success() {
            return Err(AppError::ProviderError(format!(
                "Spotify skip error: {}",
                response.status()
            )));
        }

        Ok(())
    }

    /// Get current playback info
    async fn get_current_playback(
        &self,
        _device_id: &str,
    ) -> Result<Option<ProviderTrack>, AppError> {
        let spotify_track = self.get_current_playback_internal().await?;

        Ok(spotify_track.map(|t| ProviderTrack {
            id: t.id,
            title: t.name,
            artist: t
                .artists
                .first()
                .map(|a| a.name.clone())
                .unwrap_or_default(),
            duration_ms: 0,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spotify_provider_creation() {
        let provider = SpotifyProvider::new("test_token".to_string());
        assert_eq!(provider.access_token, "test_token");
    }

    #[test]
    fn test_spotify_auth_header() {
        let provider = SpotifyProvider::new("test_token".to_string());
        let header = provider.auth_header();
        assert_eq!(header, "Bearer test_token");
    }

    #[test]
    fn test_spotify_track_parsing() {
        let track = SpotifyTrack {
            id: "123".to_string(),
            name: "Song Name".to_string(),
            artists: vec![SpotifyArtist {
                name: "Artist Name".to_string(),
            }],
            uri: "spotify:track:123".to_string(),
        };

        assert_eq!(track.id, "123");
        assert_eq!(track.name, "Song Name");
        assert_eq!(track.artists[0].name, "Artist Name");
    }

    #[test]
    fn test_spotify_device_parsing() {
        let device = SpotifyDevice {
            id: "device123".to_string(),
            name: "My Speaker".to_string(),
            is_active: true,
        };

        assert_eq!(device.id, "device123");
        assert!(device.is_active);
    }
}
