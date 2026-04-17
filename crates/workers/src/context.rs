use crate::config::WorkerConfig;
use crate::workers::queue::QueueWorker;
use application::{PlaybackService, QueueService};
use infrastructure::{RedisClient, RedisLockManager, SqlRepository};
use std::sync::Arc;

pub struct WorkerContext {
    pub queue_worker: QueueWorker,
    pub playback_service: Arc<PlaybackService>,
}

pub async fn build_worker_context(
    config: WorkerConfig,
) -> Result<WorkerContext, String> {
    let db = sea_orm::Database::connect(&config.database_url)
        .await
        .map_err(|e| format!("database connection failed: {e}"))?;

    let redis = Arc::new(
        RedisClient::new(&config.redis_url)
            .map_err(|e| format!("redis connection failed: {e}"))?,
    );

    let repo = Arc::new(SqlRepository::new(db));
    let lock_manager = Arc::new(RedisLockManager::new(redis));
    let queue_service = Arc::new(QueueService::new(repo, lock_manager));
    let playback_service =
        Arc::new(PlaybackService::new(queue_service.clone()));

    let queue_worker =
        QueueWorker::new(queue_service, config.check_interval_secs);

    Ok(WorkerContext {
        queue_worker,
        playback_service,
    })
}
