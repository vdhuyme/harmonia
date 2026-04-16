use sea_orm_migration::prelude::*;

mod m20240416_000001_create_users;
mod m20240416_000002_create_rooms;
mod m20240416_000003_create_provider_accounts;
mod m20240416_000004_create_room_mappings;
mod m20240416_000005_create_queue_items;
mod m20240416_000006_create_votes;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240416_000001_create_users::Migration),
            Box::new(m20240416_000002_create_rooms::Migration),
            Box::new(m20240416_000003_create_provider_accounts::Migration),
            Box::new(m20240416_000004_create_room_mappings::Migration),
            Box::new(m20240416_000005_create_queue_items::Migration),
            Box::new(m20240416_000006_create_votes::Migration),
        ]
    }
}
