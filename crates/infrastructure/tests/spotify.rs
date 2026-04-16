use domain::traits::MusicProvider;
use infrastructure::SpotifyProvider;

#[test]
fn test_spotify_provider_creation() {
    let provider = SpotifyProvider::new("test_token".to_string());
    // Provider created successfully
    assert!(!provider.base_url.is_empty());
}

#[test]
fn test_spotify_auth_header_format() {
    let provider = SpotifyProvider::new("my_token_12345".to_string());
    let header = provider.auth_header();

    assert!(header.starts_with("Bearer "));
    assert!(header.contains("my_token_12345"));
}

#[test]
fn test_spotify_track_conversion() {
    let spotify_track = infrastructure::providers::spotify::SpotifyTrack {
        id: "track123".to_string(),
        name: "Bohemian Rhapsody".to_string(),
        artists: vec![infrastructure::providers::spotify::SpotifyArtist {
            name: "Queen".to_string(),
        }],
        uri: "spotify:track:track123".to_string(),
    };

    assert_eq!(spotify_track.id, "track123");
    assert_eq!(spotify_track.name, "Bohemian Rhapsody");
    assert_eq!(spotify_track.artists.len(), 1);
    assert_eq!(spotify_track.artists[0].name, "Queen");
}

#[test]
fn test_spotify_device_parsing() {
    let device = infrastructure::providers::spotify::SpotifyDevice {
        id: "device_abc123".to_string(),
        name: "Living Room Speaker".to_string(),
        is_active: true,
    };

    assert_eq!(device.id, "device_abc123");
    assert_eq!(device.name, "Living Room Speaker");
    assert!(device.is_active);
}

#[test]
fn test_spotify_device_inactive() {
    let device = infrastructure::providers::spotify::SpotifyDevice {
        id: "device_inactive".to_string(),
        name: "Inactive Device".to_string(),
        is_active: false,
    };

    assert!(!device.is_active);
}

#[test]
fn test_spotify_multiple_artists() {
    let spotify_track = infrastructure::providers::spotify::SpotifyTrack {
        id: "collab_track".to_string(),
        name: "Collaboration Song".to_string(),
        artists: vec![
            infrastructure::providers::spotify::SpotifyArtist {
                name: "Artist 1".to_string(),
            },
            infrastructure::providers::spotify::SpotifyArtist {
                name: "Artist 2".to_string(),
            },
        ],
        uri: "spotify:track:collab_track".to_string(),
    };

    assert_eq!(spotify_track.artists.len(), 2);
    assert_eq!(spotify_track.artists[0].name, "Artist 1");
    assert_eq!(spotify_track.artists[1].name, "Artist 2");
}

#[test]
fn test_spotify_provider_instantiation_multiple_tokens() {
    let provider1 = SpotifyProvider::new("token1".to_string());
    let provider2 = SpotifyProvider::new("token2".to_string());

    let header1 = provider1.auth_header();
    let header2 = provider2.auth_header();

    assert_ne!(header1, header2);
    assert!(header1.contains("token1"));
    assert!(header2.contains("token2"));
}

#[tokio::test]
async fn test_spotify_provider_trait_implementation() {
    let provider = SpotifyProvider::new("test_token".to_string());

    // Test that provider implements MusicProvider
    // This is more of a compile-time check
    let _provider_ref: &dyn MusicProvider = &provider;
}
