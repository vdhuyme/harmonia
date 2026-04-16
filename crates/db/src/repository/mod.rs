use domain::{QueueItem, QueueStatus, Result, User, Room, ProviderAccount, RoomMapping, Vote};
use chrono::Utc;

/// Repository trait for generic CRUD operations
#[async_trait::async_trait]
pub trait Repository<T: Send> {
    async fn find_by_id(&self, id: &str) -> Result<T>;
    async fn create(&self, entity: T) -> Result<T>;
    async fn update(&self, entity: T) -> Result<T>;
    async fn delete(&self, id: &str) -> Result<()>;
}

/// User repository
#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<User>;
    async fn find_by_email(&self, email: &str) -> Result<User>;
    async fn create(&self, user: User) -> Result<User>;
    async fn list_all(&self) -> Result<Vec<User>>;
}

/// Room repository
#[async_trait::async_trait]
pub trait RoomRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<Room>;
    async fn create(&self, room: Room) -> Result<Room>;
    async fn list_all(&self) -> Result<Vec<Room>>;
}

/// Queue item repository
#[async_trait::async_trait]
pub trait QueueItemRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<QueueItem>;
    async fn find_by_room(&self, room_id: &str) -> Result<Vec<QueueItem>>;
    async fn find_pending_by_room(&self, room_id: &str) -> Result<Vec<QueueItem>>;
    async fn find_currently_playing(&self, room_id: &str) -> Result<Option<QueueItem>>;
    async fn create(&self, item: QueueItem) -> Result<QueueItem>;
    async fn update_status(&self, id: &str, status: QueueStatus) -> Result<()>;
    async fn update_with_timestamps(
        &self,
        id: &str,
        status: QueueStatus,
        started_at: Option<chrono::DateTime<Utc>>,
    ) -> Result<()>;
    async fn increment_votes(&self, id: &str) -> Result<i32>;
    async fn list_by_status(&self, room_id: &str, status: QueueStatus) -> Result<Vec<QueueItem>>;
}

/// Vote repository
#[async_trait::async_trait]
pub trait VoteRepository: Send + Sync {
    async fn find_vote(&self, queue_item_id: &str, user_id: &str) -> Result<Option<Vote>>;
    async fn create_vote(&self, vote: Vote) -> Result<Vote>;
    async fn delete_vote(&self, queue_item_id: &str, user_id: &str) -> Result<()>;
    async fn count_votes(&self, queue_item_id: &str) -> Result<i32>;
    async fn user_has_voted(&self, queue_item_id: &str, user_id: &str) -> Result<bool>;
}

/// Provider account repository
#[async_trait::async_trait]
pub trait ProviderAccountRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<ProviderAccount>;
    async fn find_by_user(&self, user_id: &str) -> Result<Vec<ProviderAccount>>;
    async fn create(&self, account: ProviderAccount) -> Result<ProviderAccount>;
    async fn update_token(
        &self,
        id: &str,
        access_token: String,
        expires_at: chrono::DateTime<Utc>,
    ) -> Result<()>;
}

/// Room mapping repository
#[async_trait::async_trait]
pub trait RoomMappingRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> Result<RoomMapping>;
    async fn find_by_room(&self, room_id: &str) -> Result<Vec<RoomMapping>>;
    async fn create(&self, mapping: RoomMapping) -> Result<RoomMapping>;
    async fn delete(&self, id: &str) -> Result<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_traits_exist() {
        // This test just verifies the trait definitions compile
        // Actual implementations will be tested separately
    }
}
