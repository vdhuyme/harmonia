use crate::redis::RedisClient;
use domain::error::{DomainError, DomainResult};
use domain::models::*;
use domain::rules::{calculate_priority_score, can_vote};
use domain::traits::Repository;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

pub struct QueueEngine {
    repo: Arc<dyn Repository>,
    redis: Arc<RedisClient>,
}

impl QueueEngine {
    pub fn new(repo: Arc<dyn Repository>, redis: Arc<RedisClient>) -> Self {
        Self { repo, redis }
    }

    pub async fn add_song(
        &self,
        room_id: Uuid,
        user_id: Uuid,
        track: Track,
    ) -> DomainResult<Uuid> {
        let lock_key = format!("lock:room:{}", room_id);
        let lock_value = Uuid::new_v4().to_string();

        if !self
            .redis
            .acquire_lock(&lock_key, &lock_value, Duration::from_secs(10))
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
        {
            return Err(DomainError::Conflict(
                "Room is currently being updated".to_string(),
            ));
        }

        let result = self.execute_add_song(room_id, user_id, track).await;

        let _ = self
            .redis
            .release_lock(&lock_key, &lock_value)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()));

        result
    }

    async fn execute_add_song(
        &self,
        room_id: Uuid,
        user_id: Uuid,
        track: Track,
    ) -> DomainResult<Uuid> {
        let item = QueueItem {
            id: Uuid::new_v4(),
            room_id,
            track,
            added_by: user_id,
            priority: 0, // Base priority
            votes: 0,
            played_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.repo.create_queue_item(item).await
    }

    pub async fn vote_song(
        &self,
        room_id: Uuid,
        user_id: Uuid,
        queue_item_id: Uuid,
        value: i8,
    ) -> DomainResult<()> {
        let lock_key = format!("lock:room:{}", room_id);
        let lock_value = Uuid::new_v4().to_string();

        if !self
            .redis
            .acquire_lock(&lock_key, &lock_value, Duration::from_secs(10))
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
        {
            return Err(DomainError::Conflict(
                "Room is currently being updated".to_string(),
            ));
        }

        let result = self.execute_vote(user_id, queue_item_id, value).await;

        let _ = self
            .redis
            .release_lock(&lock_key, &lock_value)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()));

        result
    }

    async fn execute_vote(
        &self,
        user_id: Uuid,
        queue_item_id: Uuid,
        value: i8,
    ) -> DomainResult<()> {
        let item = self.repo.get_queue_item_by_id(queue_item_id).await?;

        if !can_vote(&user_id, &item) {
            return Err(DomainError::ValidationError(
                "Cannot vote for this track".to_string(),
            ));
        }

        // Check if user already voted
        if self
            .repo
            .get_vote_by_user_and_queue_item(user_id, queue_item_id)
            .await
            .is_ok()
        {
            return Err(DomainError::ValidationError(
                "User already voted for this track".to_string(),
            ));
        }

        self.repo.increment_vote(queue_item_id, value).await?;

        let vote = Vote {
            id: Uuid::new_v4(),
            user_id,
            queue_item_id,
            value,
            created_at: chrono::Utc::now(),
        };

        self.repo.create_vote(vote).await?;

        Ok(())
    }

    pub async fn get_sorted_queue(
        &self,
        room_id: Uuid,
    ) -> DomainResult<Vec<QueueItem>> {
        let items = self.repo.get_queue_items_by_room_id(room_id).await?;
        let mut sorted_items = items;

        sorted_items.sort_by(|a, b| {
            let score_a = calculate_priority_score(a);
            let score_b = calculate_priority_score(b);
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(sorted_items)
    }
}
