//! Test utilities and fixtures

use domain::{
    User, Room, QueueItem, ProviderAccount, RoomMapping, Vote, 
    ProviderType, QueueStatus
};
use chrono::Utc;
use uuid::Uuid;

/// Create a test user
pub fn create_test_user(name: &str) -> User {
    User {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        email: format!("{}@test.local", name),
        role: "user".to_string(),
        created_at: Utc::now(),
    }
}

/// Create a test room
pub fn create_test_room(name: &str) -> Room {
    Room {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        created_at: Utc::now(),
    }
}

/// Create a test queue item
pub fn create_test_queue_item(
    room_id: &str,
    title: &str,
    priority: i32,
    votes: i32,
) -> QueueItem {
    QueueItem {
        id: Uuid::new_v4().to_string(),
        room_id: room_id.to_string(),
        provider: ProviderType::Spotify,
        track_id: Uuid::new_v4().to_string(),
        title: title.to_string(),
        artist: "Test Artist".to_string(),
        priority,
        votes,
        status: QueueStatus::Pending,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        started_at: None,
        ended_at: None,
    }
}

/// Create a test provider account
pub fn create_test_provider_account(user_id: &str) -> ProviderAccount {
    ProviderAccount {
        id: Uuid::new_v4().to_string(),
        provider: ProviderType::Spotify,
        user_id: user_id.to_string(),
        access_token: "test_token".to_string(),
        refresh_token: Some("test_refresh".to_string()),
        expires_at: Utc::now() + chrono::Duration::hours(1),
        created_at: Utc::now(),
    }
}

/// Create a test room mapping
pub fn create_test_room_mapping(room_id: &str, provider_account_id: &str) -> RoomMapping {
    RoomMapping {
        id: Uuid::new_v4().to_string(),
        room_id: room_id.to_string(),
        provider_account_id: provider_account_id.to_string(),
        device_id: "device_123".to_string(),
        created_at: Utc::now(),
    }
}

/// Create a test vote
pub fn create_test_vote(queue_item_id: &str, user_id: &str) -> Vote {
    Vote {
        id: Uuid::new_v4().to_string(),
        queue_item_id: queue_item_id.to_string(),
        user_id: user_id.to_string(),
        created_at: Utc::now(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_user() {
        let user = create_test_user("alice");
        assert_eq!(user.name, "alice");
        assert_eq!(user.email, "alice@test.local");
        assert_eq!(user.role, "user");
    }

    #[test]
    fn test_create_test_room() {
        let room = create_test_room("living_room");
        assert_eq!(room.name, "living_room");
        assert!(!room.id.is_empty());
    }

    #[test]
    fn test_create_test_queue_item() {
        let item = create_test_queue_item("room1", "Test Song", 50, 2);
        assert_eq!(item.title, "Test Song");
        assert_eq!(item.priority, 50);
        assert_eq!(item.votes, 2);
        assert_eq!(item.status, QueueStatus::Pending);
    }
}
