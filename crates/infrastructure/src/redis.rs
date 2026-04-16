use redis::Client;
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedisError {
    #[error("Redis operation failed: {0}")]
    OperationError(#[from] redis::RedisError),
    #[error("Lock acquisition failed")]
    LockAcquisitionError,
}

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(connection_string: &str) -> Result<Self, RedisError> {
        let client = Client::open(connection_string)?;
        Ok(Self { client })
    }

    async fn get_connection(
        &self,
    ) -> Result<redis::aio::MultiplexedConnection, RedisError> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .map_err(RedisError::OperationError)
    }

    /// Implements distributed locking using SET key value EX ttl NX
    pub async fn acquire_lock(
        &self,
        lock_key: &str,
        lock_value: &str,
        ttl: Duration,
    ) -> Result<bool, RedisError> {
        let mut conn = self.get_connection().await?;
        let result: Option<String> = redis::cmd("SET")
            .arg(lock_key)
            .arg(lock_value)
            .arg("EX")
            .arg(ttl.as_secs())
            .arg("NX")
            .query_async(&mut conn)
            .await?;

        Ok(result.is_some())
    }

    pub async fn release_lock(
        &self,
        lock_key: &str,
        lock_value: &str,
    ) -> Result<(), RedisError> {
        let mut conn = self.get_connection().await?;

        // Use a Lua script to ensure we only release the lock if we own it
        let script = redis::Script::new(
            r#"
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        "#,
        );

        script
            .key(lock_key)
            .arg(lock_value)
            .invoke_async::<()>(&mut conn)
            .await
            .map_err(RedisError::OperationError)?;

        Ok(())
    }
}
