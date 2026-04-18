pub mod m20220101_000001_create_table_users;
pub mod m20220101_000002_create_table_rooms;
pub mod m20220101_000003_create_table_provider_accounts;
pub mod m20220101_000004_create_table_queue_items;
pub mod m20220101_000005_create_table_votes;
pub mod m20220101_000006_create_table_settings;

use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table_users::Migration),
            Box::new(m20220101_000002_create_table_rooms::Migration),
            Box::new(m20220101_000003_create_table_provider_accounts::Migration),
            Box::new(m20220101_000004_create_table_queue_items::Migration),
            Box::new(m20220101_000005_create_table_votes::Migration),
            Box::new(m20220101_000006_create_table_settings::Migration),
        ]
    }
}
