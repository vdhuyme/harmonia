use chrono::Utc;
use domain::rules::*;
use domain::{ProviderType, QueueItem, QueueStatus, AppError};

#[test]
fn test_priority_score_no_votes_no_decay() {
    let now = Utc::now();
    let item = QueueItem {
        id: "test".to_string(),
        room_id: "room1".to_string(),
        provider: ProviderType::Spotify,
        track_id: "track1".to_string(),
        title: "Test Song".to_string(),
        artist: "Test Artist".to_string(),
        priority: 100,
        votes: 0,
        status: QueueStatus::Pending,
        created_at: now,
        updated_at: now,
        started_at: None,
        ended_at: None,
    };

    let score = calculate_priority_score(&item, now);
    assert_eq!(score, 100);
}

#[test]
fn test_priority_score_with_votes() {
    let now = Utc::now();
    let item = QueueItem {
        id: "test".to_string(),
        room_id: "room1".to_string(),
        provider: ProviderType::Spotify,
        track_id: "track1".to_string(),
        title: "Test Song".to_string(),
        artist: "Test Artist".to_string(),
        priority: 100,
        votes: 5,
        status: QueueStatus::Pending,
        created_at: now,
        updated_at: now,
        started_at: None,
        ended_at: None,
    };

    let score = calculate_priority_score(&item, now);
    // 100 + (5 * 10) = 150
    assert_eq!(score, 150);
}

#[test]
fn test_priority_score_with_time_decay() {
    let now = Utc::now();
    let one_hour_ago = now - chrono::Duration::hours(1);
    let item = QueueItem {
        id: "test".to_string(),
        room_id: "room1".to_string(),
        provider: ProviderType::Spotify,
        track_id: "track1".to_string(),
        title: "Test Song".to_string(),
        artist: "Test Artist".to_string(),
        priority: 100,
        votes: 0,
        status: QueueStatus::Pending,
        created_at: one_hour_ago,
        updated_at: now,
        started_at: None,
        ended_at: None,
    };

    let score = calculate_priority_score(&item, now);
    // 100 - (60 / 60) = 99
    assert_eq!(score, 99);
}

#[test]
fn test_priority_score_combined() {
    let now = Utc::now();
    let two_hours_ago = now - chrono::Duration::hours(2);
    let item = QueueItem {
        id: "test".to_string(),
        room_id: "room1".to_string(),
        provider: ProviderType::Spotify,
        track_id: "track1".to_string(),
        title: "Test Song".to_string(),
        artist: "Test Artist".to_string(),
        priority: 100,
        votes: 3,
        status: QueueStatus::Pending,
        created_at: two_hours_ago,
        updated_at: now,
        started_at: None,
        ended_at: None,
    };

    let score = calculate_priority_score(&item, now);
    // 100 + (3 * 10) - (120 / 60) = 100 + 30 - 2 = 128
    assert_eq!(score, 128);
}

#[test]
fn test_prevent_duplicate_vote() {
    let result = can_vote(true);
    assert!(result.is_err());
    match result {
        Err(AppError::DuplicateVote) => (),
        _ => panic!("Expected DuplicateVote error"),
    }
}

#[test]
fn test_allow_first_vote() {
    let result = can_vote(false);
    assert!(result.is_ok());
}