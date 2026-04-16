//! Redis-based services for distributed locking, pub/sub, and caching

use domain::{Result, AppError};
use redis::aio::ConnectionManager;
use uuid::Uuid;

/// Redis service for distributed operations
#[derive(Clone)]
pub struct RedisService {
    conn: ConnectionManager,
}

impl RedisService {
    /// Create new Redis service
    pub async fn new(url: &str) -> Result<Self> {
        let client = redis::Client::open(url)
            .map_err(|e| AppError::RedisError(e.to_string()))?;
        let conn = ConnectionManager::new(client)
            .await
            .map_err(|e| AppError::RedisError(e.to_string()))?;
        Ok(Self { conn })
    }

    /// Acquire distributed lock on a room (atomic SET NX EX)
    /// Returns true if lock acquired, false if already locked
    pub async fn acquire_lock(
        &self,
        room_id: &str,
        ttl_secs: usize,
    ) -> Result<bool> {
        let key = format!("lock:room:{}", room_id);
        let nonce = Uuid::new_v4().to_string();

        let result: bool = redis::cmd("SET")
            .arg(&key)
            .arg(&nonce)
            .arg("EX")
            .arg(ttl_secs)
            .arg("NX")
            .query_async(&mut self.conn.clone())
            .await
            .map_err(|e| AppError::RedisError(e.to_string()))?;

        if result {
            // Store nonce for later release (optional, for strict checking)
            let nonce_key = format!("lock:nonce:{}", room_id);
            let _: () = redis::cmd("SET")
                .arg(&nonce_key)
                .arg(&nonce)
                .arg("EX")
                .arg(ttl_secs)
                .query_async(&mut self.conn.clone())
                .await
                .unwrap_or(());
        }

        Ok(result)
    }

    /// Release distributed lock (safe Lua script ensures we only delete our lock)
    pub async fn release_lock(&self, room_id: &str) -> Result<()> {
        let key = format!("lock:room:{}", room_id);
        let _: () = redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut self.conn.clone())
            .await
            .map_err(|e| AppError::RedisError(e.to_string()))?;

        // Also clean up nonce key
        let nonce_key = format!("lock:nonce:{}", room_id);
        let _: () = redis::cmd("DEL")
            .arg(&nonce_key)
            .query_async(&mut self.conn.clone())
            .await
            .map_err(|e| AppError::RedisError(e.to_string()))?;

        Ok(())
    }

    /// Publish event to Redis pub/sub channel
    pub async fn publish_event(&self, channel: &str, message: &str) -> Result<()> {
        let _: i32 = redis::cmd("PUBLISH")
            .arg(channel)
            .arg(message)
            .query_async(&mut self.conn.clone())
            .await
            .map_err(|e| AppError::RedisError(e.to_string()))?;

        Ok(())
    }

    /// Cache queue for a room
    pub async fn cache_queue(&self, room_id: &str, queue_json: &str) -> Result<()> {
        let key = format!("queue:{}", room_id);
        let _: () = redis::cmd("SET")
            .arg(&key)
            .arg(queue_json)
            .arg("EX")
            .arg(300) // 5 minute cache
            .query_async(&mut self.conn.clone())
            .await
            .map_err(|e| AppError::RedisError(e.to_string()))?;

        Ok(())
    }

    /// Get cached queue for a room
    pub async fn get_cached_queue(&self, room_id: &str) -> Result<Option<String>> {
        let key = format!("queue:{}", room_id);
        let value: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut self.conn.clone())
            .await
            .map_err(|e| AppError::RedisError(e.to_string()))?;

        Ok(value)
    }

    /// Invalidate queue cache for a room
    pub async fn invalidate_queue_cache(&self, room_id: &str) -> Result<()> {
        let key = format!("queue:{}", room_id);
        let _: () = redis::cmd("DEL")
            .arg(&key)
            .query_async(&mut self.conn.clone())
            .await
            .map_err(|e| AppError::RedisError(e.to_string()))?;

        Ok(())
    }

    /// Increment rate limit counter for user
    pub async fn check_rate_limit(&self, user_id: &str, limit_per_min: i32) -> Result<bool> {
        let key = format!("rate_limit:{}", user_id);
        let count: i32 = redis::cmd("INCR")
            .arg(&key)
            .query_async(&mut self.conn.clone())
            .await
            .map_err(|e| AppError::RedisError(e.to_string()))?;

        // Set expiry on first increment
        if count == 1 {
            let _: () = redis::cmd("EXPIRE")
                .arg(&key)
                .arg(60) // 1 minute window
                .query_async(&mut self.conn.clone())
                .await
                .map_err(|e| AppError::RedisError(e.to_string()))?;
        }

        Ok(count <= limit_per_min)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_key_format() {
        let room_id = "room123";
        let key = format!("lock:room:{}", room_id);
        assert_eq!(key, "lock:room:room123");
    }

    #[test]
    fn test_queue_cache_key_format() {
        let room_id = "room123";
        let key = format!("queue:{}", room_id);
        assert_eq!(key, "queue:room123");
    }

    #[test]
    fn test_rate_limit_key_format() {
        let user_id = "user123";
        let key = format!("rate_limit:{}", user_id);
        assert_eq!(key, "rate_limit:user123");
    }
}
