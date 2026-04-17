use crate::locks::DistributedLock;
use domain::error::{DomainError, DomainResult};
use domain::models::{QueueItem, Track, Vote};
use domain::rules::{calculate_priority_score, can_vote};
use domain::traits::Repository;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

pub struct QueueService {
    repo: Arc<dyn Repository>,
    lock_manager: Arc<dyn DistributedLock>,
}

impl QueueService {
    pub fn new(
        repo: Arc<dyn Repository>,
        lock_manager: Arc<dyn DistributedLock>,
    ) -> Self {
        Self { repo, lock_manager }
    }

    pub async fn add_song(
        &self,
        room_id: Uuid,
        user_id: Uuid,
        track: Track,
    ) -> DomainResult<Uuid> {
        self.with_room_lock(room_id, async {
            let item = QueueItem {
                id: Uuid::new_v4(),
                room_id,
                track,
                added_by: user_id,
                priority: 0,
                votes: 0,
                played_at: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };

            self.repo.create_queue_item(item).await
        })
        .await
    }

    pub async fn vote_song(
        &self,
        room_id: Uuid,
        user_id: Uuid,
        queue_item_id: Uuid,
        value: i8,
    ) -> DomainResult<()> {
        self.with_room_lock(room_id, async {
            let item = self.repo.get_queue_item_by_id(queue_item_id).await?;

            if !can_vote(&user_id, &item) {
                return Err(DomainError::ValidationError(
                    "Cannot vote for this track".to_string(),
                ));
            }

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
        })
        .await
    }

    pub async fn get_sorted_queue(
        &self,
        room_id: Uuid,
    ) -> DomainResult<Vec<QueueItem>> {
        let mut items = self.repo.get_queue_items_by_room_id(room_id).await?;

        items.sort_by(|a, b| {
            let score_a = calculate_priority_score(a);
            let score_b = calculate_priority_score(b);
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(items)
    }

    async fn with_room_lock<T, F>(
        &self,
        room_id: Uuid,
        op: F,
    ) -> DomainResult<T>
    where
        F: std::future::Future<Output = DomainResult<T>>,
    {
        let lock_key = format!("lock:room:{}", room_id);
        let lock_value = Uuid::new_v4().to_string();

        let acquired = self
            .lock_manager
            .acquire_lock(&lock_key, &lock_value, Duration::from_secs(10))
            .await?;

        if !acquired {
            return Err(DomainError::Conflict(
                "Room is currently being updated".to_string(),
            ));
        }

        let result = op.await;
        let _ = self.lock_manager.release_lock(&lock_key, &lock_value).await;

        result
    }
}
