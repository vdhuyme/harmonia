use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "queue_items")]
pub struct Model {
    #[sea_orm(column_name = "id", primary_key)]
    pub id: Uuid,
    #[sea_orm(column_name = "room_id")]
    pub room_id: Uuid,
    #[sea_orm(column_name = "track_id")]
    pub track_id: String,
    #[sea_orm(column_name = "provider")]
    pub provider: String,
    #[sea_orm(column_name = "title")]
    pub title: String,
    #[sea_orm(column_name = "artist")]
    pub artist: String,
    #[sea_orm(column_name = "uri")]
    pub uri: String,
    #[sea_orm(column_name = "added_by")]
    pub added_by: Uuid,
    #[sea_orm(column_name = "priority")]
    pub priority: i32,
    #[sea_orm(column_name = "votes")]
    pub votes: i32,
    #[sea_orm(column_name = "played_at")]
    pub played_at: Option<DateTimeUtc>,
    #[sea_orm(column_name = "created_at")]
    pub created_at: DateTimeUtc,
    #[sea_orm(column_name = "updated_at")]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::rooms::Entity",
        from = "Column::RoomId",
        to = "super::rooms::Column::Id"
    )]
    Room,
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::AddedBy",
        to = "super::users::Column::Id"
    )]
    User,
}

impl Related<super::rooms::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Room.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
