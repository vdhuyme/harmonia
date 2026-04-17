use workers::config::WorkerConfig;
use workers::context::build_worker_context;
use workers::workers::playback::PlaybackWorker;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = WorkerConfig::from_env();
    let context = build_worker_context(config)
        .await
        .expect("failed to initialize workers");

    let worker =
        PlaybackWorker::new(context.queue_worker, context.playback_service);
    worker.start().await;
}
