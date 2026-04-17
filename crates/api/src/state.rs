use crate::config::Config;
use application::{PlaybackService, QueueService};
use infrastructure::{
    ProviderResolver, RedisClient, RedisLockManager, SecurityService,
    SpotifyProvider, SqlRepository, YouTubeProvider,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub queue_service: Arc<QueueService>,
    pub playback_service: Arc<PlaybackService>,
    pub security: Arc<SecurityService>,
    #[allow(dead_code)]
    pub provider_resolver: Arc<ProviderResolver>,
    pub config: Arc<Config>,
}

pub async fn build_app_state(config: Config) -> Result<AppState, String> {
    let db = sea_orm::Database::connect(&config.database_url)
        .await
        .map_err(|e| format!("database connection failed: {e}"))?;

    let redis = Arc::new(
        RedisClient::new(&config.redis_url)
            .map_err(|e| format!("redis connection failed: {e}"))?,
    );

    let security = Arc::new(SecurityService::new(
        config.jwt_secret.clone(),
        vec![0u8; 32],
    ));

    let repo = Arc::new(SqlRepository::new(db));
    let lock_manager = Arc::new(RedisLockManager::new(redis));
    let queue_service = Arc::new(QueueService::new(repo, lock_manager));
    let playback_service =
        Arc::new(PlaybackService::new(queue_service.clone()));

    let spotify_provider = Arc::new(SpotifyProvider::new(security.clone()));
    let youtube_provider =
        Arc::new(YouTubeProvider::new(config.youtube_api_key.clone()));
    let provider_resolver =
        Arc::new(ProviderResolver::new(spotify_provider, youtube_provider));

    Ok(AppState {
        queue_service,
        playback_service,
        security,
        provider_resolver,
        config: Arc::new(config),
    })
}
