use super::queue::QueueService;
use domain::error::{DomainError, DomainResult};
use domain::models::QueueItem;
use std::sync::Arc;
use uuid::Uuid;

pub struct PlaybackService {
    queue_service: Arc<QueueService>,
}

#[derive(Debug, Clone)]
pub struct PlaybackDecision {
    pub item: QueueItem,
}

impl PlaybackService {
    pub fn new(queue_service: Arc<QueueService>) -> Self {
        Self { queue_service }
    }

    pub async fn next_track(
        &self,
        room_id: Uuid,
    ) -> DomainResult<PlaybackDecision> {
        let queue = self.queue_service.get_sorted_queue(room_id).await?;

        let item = queue.into_iter().next().ok_or_else(|| {
            DomainError::NotFound("Queue is empty".to_string())
        })?;

        Ok(PlaybackDecision { item })
    }
}
