use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize,
)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(column_name = "id", primary_key)]
    pub id: Uuid,
    #[sea_orm(column_name = "username")]
    pub username: String,
    #[sea_orm(column_name = "email")]
    pub email: String,
    #[sea_orm(column_name = "created_at")]
    pub created_at: DateTimeUtc,
    #[sea_orm(column_name = "updated_at")]
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
