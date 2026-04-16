use domain::{AppError, QueueItem, QueueStatus, Result};
use chrono::Utc;
use crate::redis_service::RedisService;

/// Queue engine for managing playback and priority
#[derive(Clone)]
pub struct QueueEngine {
    redis: RedisService,
}

impl QueueEngine {
    pub fn new(redis: RedisService) -> Self {
        Self { redis }
    }

    /// Calculate priority score for queue item
    /// Score = base_priority + (votes * 10) - (minutes_since_creation / 60)
    pub fn calculate_priority_score(item: &QueueItem) -> i32 {
        let now = Utc::now();
        let minutes_since_creation = (now - item.created_at).num_minutes() as i32;
        let time_decay = minutes_since_creation / 60;
        item.priority + (item.votes * 10) - time_decay
    }

    /// Select next song from pending queue items
    /// Returns None if no pending items, Some(QueueItem) if found
    /// Note: This is a pure logic function; actual DB query happens in repository
    pub fn select_next_song(pending_items: &[QueueItem]) -> Option<QueueItem> {
        if pending_items.is_empty() {
            return None;
        }

        // Find item with highest priority score
        pending_items
            .iter()
            .max_by_key(|item| Self::calculate_priority_score(item))
            .cloned()
    }

    /// Validate song can be played
    pub fn validate_song_for_playback(item: &QueueItem) -> Result<()> {
        if item.status != QueueStatus::Pending {
            return Err(AppError::ValidationFailed(
                format!("Song status is {}, not pending", item.status.as_str()),
            ));
        }

        if item.track_id.is_empty() {
            return Err(AppError::ValidationFailed(
                "Track ID is empty".to_string(),
            ));
        }

        Ok(())
    }

    /// Attempt to acquire queue lock for a room
    pub async fn acquire_queue_lock(&self, room_id: &str, ttl_secs: usize) -> Result<bool> {
        self.redis.acquire_lock(room_id, ttl_secs).await
    }

    /// Release queue lock for a room
    pub async fn release_queue_lock(&self, room_id: &str) -> Result<()> {
        self.redis.release_lock(room_id).await
    }

    /// Broadcast queue updated event
    pub async fn broadcast_queue_updated(&self, room_id: &str) -> Result<()> {
        let channel = format!("event:queue_updated:{}", room_id);
        self.redis.publish_event(&channel, "queue_updated").await
    }

    /// Broadcast song started event
    pub async fn broadcast_song_started(&self, room_id: &str, track_id: &str) -> Result<()> {
        let channel = format!("event:song_started:{}", room_id);
        self.redis.publish_event(&channel, track_id).await
    }

    /// Broadcast song finished event
    pub async fn broadcast_song_finished(&self, room_id: &str) -> Result<()> {
        let channel = format!("event:song_finished:{}", room_id);
        self.redis.publish_event(&channel, "song_finished").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::ProviderType;

    fn create_test_item(id: &str, priority: i32, votes: i32, age_minutes: i64) -> QueueItem {
        let now = Utc::now();
        let created_at = now - chrono::Duration::minutes(age_minutes);

        QueueItem {
            id: id.to_string(),
            room_id: "room1".to_string(),
            provider: ProviderType::Spotify,
            track_id: format!("track_{}", id),
            title: format!("Song {}", id),
            artist: "Test Artist".to_string(),
            priority,
            votes,
            status: QueueStatus::Pending,
            created_at,
            updated_at: now,
            started_at: None,
            ended_at: None,
        }
    }

    #[test]
    fn test_calculate_priority_no_votes_no_decay() {
        let item = create_test_item("1", 100, 0, 0);
        let score = QueueEngine::calculate_priority_score(&item);
        assert_eq!(score, 100);
    }

    #[test]
    fn test_calculate_priority_with_votes() {
        let item = create_test_item("1", 100, 5, 0);
        let score = QueueEngine::calculate_priority_score(&item);
        assert_eq!(score, 150); // 100 + (5 * 10)
    }

    #[test]
    fn test_calculate_priority_with_decay() {
        let item = create_test_item("1", 100, 0, 60);
        let score = QueueEngine::calculate_priority_score(&item);
        assert_eq!(score, 99); // 100 - (60 / 60)
    }

    #[test]
    fn test_select_next_song_empty() {
        let items = vec![];
        let result = QueueEngine::select_next_song(&items);
        assert!(result.is_none());
    }

    #[test]
    fn test_select_next_song_single() {
        let items = vec![create_test_item("1", 100, 0, 0)];
        let result = QueueEngine::select_next_song(&items);
        assert!(result.is_some());
        assert_eq!(result.unwrap().id, "1");
    }

    #[test]
    fn test_select_next_song_by_priority() {
        let items = vec![
            create_test_item("1", 50, 0, 0),
            create_test_item("2", 100, 0, 0),
            create_test_item("3", 75, 0, 0),
        ];
        let result = QueueEngine::select_next_song(&items);
        assert_eq!(result.unwrap().id, "2"); // Highest priority
    }

    #[test]
    fn test_select_next_song_by_votes() {
        let items = vec![
            create_test_item("1", 100, 1, 0),
            create_test_item("2", 100, 5, 0),
            create_test_item("3", 100, 2, 0),
        ];
        let result = QueueEngine::select_next_song(&items);
        assert_eq!(result.unwrap().id, "2"); // Most votes: 100 + (5*10) = 150
    }

    #[test]
    fn test_select_next_song_with_time_decay() {
        let items = vec![
            create_test_item("1", 100, 0, 0), // Score: 100
            create_test_item("2", 100, 0, 60), // Score: 99 (old)
        ];
        let result = QueueEngine::select_next_song(&items);
        assert_eq!(result.unwrap().id, "1"); // Newer item wins
    }

    #[test]
    fn test_validate_song_success() {
        let item = create_test_item("1", 100, 0, 0);
        assert!(QueueEngine::validate_song_for_playback(&item).is_ok());
    }

    #[test]
    fn test_validate_song_wrong_status() {
        let mut item = create_test_item("1", 100, 0, 0);
        item.status = QueueStatus::Playing;
        let result = QueueEngine::validate_song_for_playback(&item);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_song_empty_track_id() {
        let mut item = create_test_item("1", 100, 0, 0);
        item.track_id = String::new();
        let result = QueueEngine::validate_song_for_playback(&item);
        assert!(result.is_err());
    }
}
