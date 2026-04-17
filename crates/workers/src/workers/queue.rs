use application::QueueService;
use std::sync::Arc;
use std::time::Duration;

pub struct QueueWorker {
    queue_service: Arc<QueueService>,
    check_interval_secs: u64,
}

impl QueueWorker {
    pub fn new(
        queue_service: Arc<QueueService>,
        check_interval_secs: u64,
    ) -> Self {
        Self {
            queue_service,
            check_interval_secs,
        }
    }

    pub async fn start(&self) {
        loop {
            self.process_queues().await;
            tokio::time::sleep(Duration::from_secs(self.check_interval_secs))
                .await;
        }
    }

    async fn process_queues(&self) {
        let _ = &self.queue_service;
        tracing::info!("queue worker processed polling cycle");
    }
}
