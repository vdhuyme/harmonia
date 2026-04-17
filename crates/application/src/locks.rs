use async_trait::async_trait;
use domain::error::DomainResult;
use std::time::Duration;

#[async_trait]
pub trait DistributedLock: Send + Sync {
    async fn acquire_lock(
        &self,
        key: &str,
        value: &str,
        ttl: Duration,
    ) -> DomainResult<bool>;

    async fn release_lock(&self, key: &str, value: &str) -> DomainResult<()>;
}
