use domain::{ProviderType, QueueStatus};

#[test]
fn test_queue_status_as_str() {
    assert_eq!(QueueStatus::Pending.as_str(), "pending");
    assert_eq!(QueueStatus::Playing.as_str(), "playing");
    assert_eq!(QueueStatus::Skipped.as_str(), "skipped");
    assert_eq!(QueueStatus::Done.as_str(), "done");
}

#[test]
fn test_queue_status_from_str() {
    assert_eq!(
        QueueStatus::from_str("pending").unwrap(),
        QueueStatus::Pending
    );
    assert_eq!(
        QueueStatus::from_str("playing").unwrap(),
        QueueStatus::Playing
    );
    assert_eq!(
        QueueStatus::from_str("skipped").unwrap(),
        QueueStatus::Skipped
    );
    assert_eq!(QueueStatus::from_str("done").unwrap(), QueueStatus::Done);
}

#[test]
fn test_queue_status_from_str_invalid() {
    let result = QueueStatus::from_str("invalid");
    assert!(result.is_err());
}

#[test]
fn test_provider_type_as_str() {
    assert_eq!(ProviderType::Spotify.as_str(), "spotify");
    assert_eq!(ProviderType::YouTube.as_str(), "youtube");
}

#[test]
fn test_provider_type_from_str() {
    assert_eq!(
        ProviderType::from_str("spotify").unwrap(),
        ProviderType::Spotify
    );
    assert_eq!(
        ProviderType::from_str("youtube").unwrap(),
        ProviderType::YouTube
    );
}

#[test]
fn test_provider_type_from_str_invalid() {
    let result = ProviderType::from_str("invalid");
    assert!(result.is_err());
}
