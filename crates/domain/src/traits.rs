use crate::error::DomainResult;
use crate::models::{ProviderAccount, QueueItem, Room, User, Vote};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait Repository: Send + Sync {
    async fn get_user_by_id(&self, id: Uuid) -> DomainResult<User>;
    async fn get_user_by_email(&self, email: String) -> DomainResult<User>;
    async fn create_user(&self, user: User) -> DomainResult<Uuid>;
    async fn update_user(&self, user: User) -> DomainResult<()>;
    async fn delete_user(&self, id: Uuid) -> DomainResult<()>;

    async fn get_provider_account_by_user_id(
        &self,
        user_id: Uuid,
    ) -> DomainResult<ProviderAccount>;
    async fn create_provider_account(
        &self,
        account: ProviderAccount,
    ) -> DomainResult<Uuid>;
    async fn update_provider_account(
        &self,
        account: ProviderAccount,
    ) -> DomainResult<()>;
    async fn delete_provider_account(&self, id: Uuid) -> DomainResult<()>;

    async fn get_room_by_id(&self, id: Uuid) -> DomainResult<Room>;
    async fn get_room_by_name(&self, name: String) -> DomainResult<Room>;
    async fn create_room(&self, room: Room) -> DomainResult<Uuid>;
    async fn update_room(&self, room: Room) -> DomainResult<()>;
    async fn delete_room(&self, id: Uuid) -> DomainResult<()>;

    async fn get_queue_item_by_id(&self, id: Uuid) -> DomainResult<QueueItem>;
    async fn get_queue_items_by_room_id(
        &self,
        room_id: Uuid,
    ) -> DomainResult<Vec<QueueItem>>;
    async fn create_queue_item(&self, item: QueueItem) -> DomainResult<Uuid>;
    async fn update_queue_item(&self, item: QueueItem) -> DomainResult<()>;
    async fn delete_queue_item(&self, id: Uuid) -> DomainResult<()>;
    async fn increment_vote(
        &self,
        queue_item_id: Uuid,
        value: i8,
    ) -> DomainResult<()>;

    async fn get_vote_by_user_and_queue_item(
        &self,
        user_id: Uuid,
        queue_item_id: Uuid,
    ) -> DomainResult<Vote>;
    async fn create_vote(&self, vote: Vote) -> DomainResult<Uuid>;
    async fn delete_vote(&self, id: Uuid) -> DomainResult<()>;
}
