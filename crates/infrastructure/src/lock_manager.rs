use application::locks::DistributedLock;
use async_trait::async_trait;
use domain::error::{DomainError, DomainResult};
use std::sync::Arc;
use std::time::Duration;

use crate::redis::RedisClient;

pub struct RedisLockManager {
    redis: Arc<RedisClient>,
}

impl RedisLockManager {
    pub fn new(redis: Arc<RedisClient>) -> Self {
        Self { redis }
    }
}

#[async_trait]
impl DistributedLock for RedisLockManager {
    async fn acquire_lock(
        &self,
        key: &str,
        value: &str,
        ttl: Duration,
    ) -> DomainResult<bool> {
        self.redis
            .acquire_lock(key, value, ttl)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn release_lock(&self, key: &str, value: &str) -> DomainResult<()> {
        self.redis
            .release_lock(key, value)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }
}
