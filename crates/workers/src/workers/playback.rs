use crate::workers::queue::QueueWorker;
use application::PlaybackService;
use std::sync::Arc;

pub struct PlaybackWorker {
    queue_worker: QueueWorker,
    playback_service: Arc<PlaybackService>,
}

impl PlaybackWorker {
    pub fn new(
        queue_worker: QueueWorker,
        playback_service: Arc<PlaybackService>,
    ) -> Self {
        Self {
            queue_worker,
            playback_service,
        }
    }

    pub async fn start(&self) {
        let _ = &self.playback_service;
        self.queue_worker.start().await;
    }
}
