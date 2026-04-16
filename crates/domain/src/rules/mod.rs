pub mod voting;

use crate::{QueueItem, Result};

/// Calculate priority score for queue item
/// Score = base_priority + (votes * 10) - (minutes_since_creation / 60)
/// This ensures votes matter but old songs don't languish at bottom forever
pub fn calculate_priority_score(
    item: &QueueItem,
    now: chrono::DateTime<chrono::Utc>,
) -> i32 {
    let minutes_since_creation = (now - item.created_at).num_minutes() as i32;
    let time_decay = minutes_since_creation / 60;
    item.priority + (item.votes * 10) - time_decay
}

/// Validate if user can vote for queue item
pub fn can_vote(existing_vote_by_user: bool) -> Result<()> {
    if existing_vote_by_user {
        return Err(crate::AppError::DuplicateVote);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ProviderType, QueueStatus};
    use chrono::Utc;

    #[test]
    fn test_priority_calculation_baseline() {
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
        assert_eq!(score, 100); // No time decay, no votes
    }

    #[test]
    fn test_priority_calculation_with_votes() {
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
        assert_eq!(score, 150); // 100 + (5 * 10)
    }

    #[test]
    fn test_priority_calculation_with_time_decay() {
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
        assert_eq!(score, 99); // 100 - (60 / 60)
    }

    #[test]
    fn test_can_vote_prevents_duplicates() {
        let result = can_vote(true);
        assert!(result.is_err());
        match result {
            Err(crate::AppError::DuplicateVote) => (),
            _ => panic!("Expected DuplicateVote error"),
        }
    }

    #[test]
    fn test_can_vote_allows_first_vote() {
        let result = can_vote(false);
        assert!(result.is_ok());
    }
}
