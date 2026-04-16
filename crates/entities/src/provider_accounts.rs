use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "provider_accounts")]
pub struct Model {
    #[sea_orm(column_name = "id", primary_key)]
    pub id: Uuid,
    #[sea_orm(column_name = "user_id")]
    pub user_id: Uuid,
    #[sea_orm(column_name = "provider")]
    pub provider: String, // Store as string for enum
    #[sea_orm(column_name = "provider_user_id")]
    pub provider_user_id: String,
    #[sea_orm(column_name = "access_token")]
    pub access_token: String,
    #[sea_orm(column_name = "refresh_token")]
    pub refresh_token: Option<String>,
    #[sea_orm(column_name = "expires_at")]
    pub expires_at: Option<DateTimeUtc>,
    #[sea_orm(column_name = "created_at")]
    pub created_at: DateTimeUtc,
    #[sea_orm(column_name = "updated_at")]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
}

impl ActiveModelBehavior for ActiveModel {}
