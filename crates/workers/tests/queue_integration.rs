//! Integration tests for queue engine workflows

use chrono::Utc;
use domain::{ProviderType, QueueItem, QueueStatus};
use workers::QueueEngine;

fn create_queue_item(id: &str, priority: i32, votes: i32) -> QueueItem {
    QueueItem {
        id: id.to_string(),
        room_id: "room1".to_string(),
        provider: ProviderType::Spotify,
        track_id: format!("track_{}", id),
        title: format!("Song {}", id),
        artist: "Artist".to_string(),
        priority,
        votes,
        status: QueueStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        started_at: None,
        ended_at: None,
    }
}

#[test]
fn test_queue_selection_workflow() {
    // Create a mock queue with multiple songs
    let queue = vec![
        create_queue_item("song1", 10, 0), // Low priority
        create_queue_item("song2", 50, 2), // Medium + votes
        create_queue_item("song3", 100, 1), // High priority + vote
    ];

    // Select next song
    let next = QueueEngine::select_next_song(&queue);
    assert!(next.is_some());
    let song = next.unwrap();
    assert_eq!(song.id, "song3"); // Highest score: 100 + (1*10) = 110
}

#[test]
fn test_queue_validation() {
    let valid_item = create_queue_item("valid", 50, 0);
    assert!(QueueEngine::validate_song_for_playback(&valid_item).is_ok());

    let mut invalid_item = create_queue_item("invalid", 50, 0);
    invalid_item.status = QueueStatus::Playing;
    assert!(QueueEngine::validate_song_for_playback(&invalid_item).is_err());
}

#[test]
fn test_priority_tiebreaker_with_votes() {
    let queue = vec![
        create_queue_item("song1", 100, 2), // Score: 100 + 20 = 120
        create_queue_item("song2", 100, 2), // Score: 100 + 20 = 120 (tie)
        create_queue_item("song3", 100, 1), // Score: 100 + 10 = 110
    ];

    let next = QueueEngine::select_next_song(&queue);
    // In case of tie, max_by_key returns first occurrence
    assert!(next.is_some());
    assert_eq!(next.unwrap().id, "song1");
}
