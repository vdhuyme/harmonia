use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "votes")]
pub struct Model {
    #[sea_orm(column_name = "id", primary_key)]
    pub id: Uuid,
    #[sea_orm(column_name = "user_id")]
    pub user_id: Uuid,
    #[sea_orm(column_name = "queue_item_id")]
    pub queue_item_id: Uuid,
    #[sea_orm(column_name = "value")]
    pub value: i8,
    #[sea_orm(column_name = "created_at")]
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
    #[sea_orm(
        belongs_to = "super::queue_items::Entity",
        from = "Column::QueueItemId",
        to = "super::queue_items::Column::Id"
    )]
    QueueItem,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::queue_items::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::QueueItem.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
