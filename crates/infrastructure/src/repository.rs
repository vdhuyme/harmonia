use async_trait::async_trait;
use domain::error::DomainError;
use domain::models::*;
use domain::traits::Repository;
use sea_orm::*;
use uuid::Uuid;

pub struct SqlRepository {
    db: DatabaseConnection,
}

impl SqlRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }
}

#[async_trait]
impl Repository for SqlRepository {
    async fn get_user_by_id(
        &self,
        id: Uuid,
    ) -> domain::error::DomainResult<User> {
        let user = entities::users::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
            .ok_or(DomainError::NotFound("User not found".to_string()))?;

        Ok(User {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    async fn get_user_by_email(
        &self,
        email: String,
    ) -> domain::error::DomainResult<User> {
        let user = entities::users::Entity::find()
            .filter(entities::users::Column::Email.eq(email.clone()))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
            .ok_or(DomainError::NotFound(format!(
                "User with email {} not found",
                email
            )))?;

        Ok(User {
            id: user.id,
            username: user.username,
            email: user.email,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    async fn create_user(
        &self,
        user: User,
    ) -> domain::error::DomainResult<Uuid> {
        let active_model = entities::users::ActiveModel {
            id: Set(user.id),
            username: Set(user.username),
            email: Set(user.email),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
        };

        entities::users::Entity::insert(active_model)
            .exec(&self.db)
            .await
            .map(|_| user.id)
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn update_user(&self, user: User) -> domain::error::DomainResult<()> {
        let active_model = entities::users::ActiveModel {
            id: Set(user.id),
            username: Set(user.username),
            email: Set(user.email),
            updated_at: Set(user.updated_at),
            ..Default::default()
        };

        entities::users::Entity::update(active_model)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn delete_user(&self, id: Uuid) -> domain::error::DomainResult<()> {
        entities::users::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn get_provider_account_by_user_id(
        &self,
        user_id: Uuid,
    ) -> domain::error::DomainResult<ProviderAccount> {
        let account = entities::provider_accounts::Entity::find()
            .filter(entities::provider_accounts::Column::UserId.eq(user_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
            .ok_or(DomainError::NotFound(
                "Provider account not found".to_string(),
            ))?;

        // Convert string to MusicProvider enum
        let provider = match account.provider.as_str() {
            "spotify" => MusicProvider::Spotify,
            "youtube" => MusicProvider::YouTube,
            _ => {
                return Err(DomainError::InvalidInput(format!(
                    "Unknown provider: {}",
                    account.provider
                )))
            }
        };

        Ok(ProviderAccount {
            id: account.id,
            user_id: account.user_id,
            provider,
            provider_user_id: account.provider_user_id,
            access_token: account.access_token,
            refresh_token: account.refresh_token,
            expires_at: account.expires_at,
            created_at: account.created_at,
            updated_at: account.updated_at,
        })
    }

    async fn create_provider_account(
        &self,
        account: ProviderAccount,
    ) -> domain::error::DomainResult<Uuid> {
        let active_model = entities::provider_accounts::ActiveModel {
            id: Set(account.id),
            user_id: Set(account.user_id),
            provider: Set(account.provider.as_str().to_string()),
            provider_user_id: Set(account.provider_user_id),
            access_token: Set(account.access_token),
            refresh_token: Set(account.refresh_token),
            expires_at: Set(account.expires_at),
            created_at: Set(account.created_at),
            updated_at: Set(account.updated_at),
        };

        entities::provider_accounts::Entity::insert(active_model)
            .exec(&self.db)
            .await
            .map(|_| account.id)
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn update_provider_account(
        &self,
        account: ProviderAccount,
    ) -> domain::error::DomainResult<()> {
        let active_model = entities::provider_accounts::ActiveModel {
            id: Set(account.id),
            access_token: Set(account.access_token),
            refresh_token: Set(account.refresh_token),
            expires_at: Set(account.expires_at),
            updated_at: Set(account.updated_at),
            ..Default::default()
        };

        entities::provider_accounts::Entity::update(active_model)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn delete_provider_account(
        &self,
        id: Uuid,
    ) -> domain::error::DomainResult<()> {
        entities::provider_accounts::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn get_room_by_id(
        &self,
        id: Uuid,
    ) -> domain::error::DomainResult<Room> {
        let room = entities::rooms::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
            .ok_or(DomainError::NotFound("Room not found".to_string()))?;

        Ok(Room {
            id: room.id,
            name: room.name,
            created_by: room.created_by,
            is_active: room.is_active,
            created_at: room.created_at,
            updated_at: room.updated_at,
        })
    }

    async fn get_room_by_name(
        &self,
        name: String,
    ) -> domain::error::DomainResult<Room> {
        let room = entities::rooms::Entity::find()
            .filter(entities::rooms::Column::Name.eq(name.clone()))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
            .ok_or(DomainError::NotFound(format!(
                "Room with name {} not found",
                name
            )))?;

        Ok(Room {
            id: room.id,
            name: room.name,
            created_by: room.created_by,
            is_active: room.is_active,
            created_at: room.created_at,
            updated_at: room.updated_at,
        })
    }

    async fn create_room(
        &self,
        room: Room,
    ) -> domain::error::DomainResult<Uuid> {
        let active_model = entities::rooms::ActiveModel {
            id: Set(room.id),
            name: Set(room.name),
            created_by: Set(room.created_by),
            is_active: Set(room.is_active),
            created_at: Set(room.created_at),
            updated_at: Set(room.updated_at),
        };

        entities::rooms::Entity::insert(active_model)
            .exec(&self.db)
            .await
            .map(|_| room.id)
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn update_room(&self, room: Room) -> domain::error::DomainResult<()> {
        let active_model = entities::rooms::ActiveModel {
            id: Set(room.id),
            name: Set(room.name),
            is_active: Set(room.is_active),
            updated_at: Set(room.updated_at),
            ..Default::default()
        };

        entities::rooms::Entity::update(active_model)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn delete_room(&self, id: Uuid) -> domain::error::DomainResult<()> {
        entities::rooms::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn get_queue_item_by_id(
        &self,
        id: Uuid,
    ) -> domain::error::DomainResult<QueueItem> {
        let item = entities::queue_items::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
            .ok_or(DomainError::NotFound("Queue item not found".to_string()))?;

        // Convert string provider to MusicProvider enum
        let provider = match item.provider.as_str() {
            "spotify" => MusicProvider::Spotify,
            "youtube" => MusicProvider::YouTube,
            _ => MusicProvider::Spotify, // Default fallback
        };

        let track = Track {
            id: item.track_id,
            provider,
            title: item.title,
            artist: item.artist,
            album: None,
            duration_ms: 0, // Not stored in entity
            uri: item.uri,
            artwork_url: None,
        };

        Ok(QueueItem {
            id: item.id,
            room_id: item.room_id,
            track,
            added_by: item.added_by,
            priority: item.priority,
            votes: item.votes,
            played_at: item.played_at,
            created_at: item.created_at,
            updated_at: item.updated_at,
        })
    }

    async fn get_queue_items_by_room_id(
        &self,
        room_id: Uuid,
    ) -> domain::error::DomainResult<Vec<QueueItem>> {
        let items = entities::queue_items::Entity::find()
            .filter(entities::queue_items::Column::RoomId.eq(room_id))
            .all(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?;

        Ok(items
            .into_iter()
            .map(|i| {
                // Convert string provider to MusicProvider enum
                let provider = match i.provider.as_str() {
                    "spotify" => MusicProvider::Spotify,
                    "youtube" => MusicProvider::YouTube,
                    _ => MusicProvider::Spotify, // Default fallback
                };

                let track = Track {
                    id: i.track_id,
                    provider,
                    title: i.title,
                    artist: i.artist,
                    album: None,
                    duration_ms: 0,
                    uri: i.uri,
                    artwork_url: None,
                };

                QueueItem {
                    id: i.id,
                    room_id: i.room_id,
                    track,
                    added_by: i.added_by,
                    priority: i.priority,
                    votes: i.votes,
                    played_at: i.played_at,
                    created_at: i.created_at,
                    updated_at: i.updated_at,
                }
            })
            .collect())
    }

    async fn create_queue_item(
        &self,
        item: QueueItem,
    ) -> domain::error::DomainResult<Uuid> {
        let active_model = entities::queue_items::ActiveModel {
            id: Set(item.id),
            room_id: Set(item.room_id),
            track_id: Set(item.track.id),
            provider: Set(item.track.provider.as_str().to_string()),
            title: Set(item.track.title),
            artist: Set(item.track.artist),
            uri: Set(item.track.uri),
            added_by: Set(item.added_by),
            priority: Set(item.priority),
            votes: Set(item.votes),
            played_at: Set(item.played_at),
            created_at: Set(item.created_at),
            updated_at: Set(item.updated_at),
        };

        entities::queue_items::Entity::insert(active_model)
            .exec(&self.db)
            .await
            .map(|_| item.id)
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn update_queue_item(
        &self,
        item: QueueItem,
    ) -> domain::error::DomainResult<()> {
        let active_model = entities::queue_items::ActiveModel {
            id: Set(item.id),
            priority: Set(item.priority),
            votes: Set(item.votes),
            played_at: Set(item.played_at),
            updated_at: Set(item.updated_at),
            ..Default::default()
        };

        entities::queue_items::Entity::update(active_model)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn delete_queue_item(
        &self,
        id: Uuid,
    ) -> domain::error::DomainResult<()> {
        entities::queue_items::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn increment_vote(
        &self,
        queue_item_id: Uuid,
        value: i8,
    ) -> domain::error::DomainResult<()> {
        let item = entities::queue_items::Entity::find_by_id(queue_item_id)
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
            .ok_or(DomainError::NotFound("Queue item not found".to_string()))?;

        let new_votes = item.votes + (value as i32);
        let active_model = entities::queue_items::ActiveModel {
            id: Set(queue_item_id),
            votes: Set(new_votes),
            ..Default::default()
        };

        entities::queue_items::Entity::update(active_model)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn get_vote_by_user_and_queue_item(
        &self,
        user_id: Uuid,
        queue_item_id: Uuid,
    ) -> domain::error::DomainResult<Vote> {
        let vote = entities::votes::Entity::find()
            .filter(entities::votes::Column::UserId.eq(user_id))
            .filter(entities::votes::Column::QueueItemId.eq(queue_item_id))
            .one(&self.db)
            .await
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))?
            .ok_or(DomainError::NotFound("Vote not found".to_string()))?;

        Ok(Vote {
            id: vote.id,
            user_id: vote.user_id,
            queue_item_id: vote.queue_item_id,
            value: vote.value,
            created_at: vote.created_at,
        })
    }

    async fn create_vote(
        &self,
        vote: Vote,
    ) -> domain::error::DomainResult<Uuid> {
        let active_model = entities::votes::ActiveModel {
            id: Set(vote.id),
            user_id: Set(vote.user_id),
            queue_item_id: Set(vote.queue_item_id),
            value: Set(vote.value),
            created_at: Set(vote.created_at),
        };

        entities::votes::Entity::insert(active_model)
            .exec(&self.db)
            .await
            .map(|_| vote.id)
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }

    async fn delete_vote(&self, id: Uuid) -> domain::error::DomainResult<()> {
        entities::votes::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| DomainError::InfrastructureError(e.to_string()))
    }
}
