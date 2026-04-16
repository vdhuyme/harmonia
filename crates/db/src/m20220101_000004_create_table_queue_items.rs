use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(QueueItems::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(QueueItems::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(QueueItems::RoomId).uuid().not_null())
                    .col(
                        ColumnDef::new(QueueItems::TrackId).string().not_null(),
                    )
                    .col(
                        ColumnDef::new(QueueItems::Provider)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(QueueItems::Title).string().not_null())
                    .col(ColumnDef::new(QueueItems::Artist).string().not_null())
                    .col(ColumnDef::new(QueueItems::Uri).string().not_null())
                    .col(ColumnDef::new(QueueItems::AddedBy).uuid().not_null())
                    .col(
                        ColumnDef::new(QueueItems::Priority)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(QueueItems::Votes)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(QueueItems::PlayedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(QueueItems::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(QueueItems::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_queue_items_room_id")
                            .from(QueueItems::Table, QueueItems::RoomId)
                            .to(Rooms::Table, Rooms::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_queue_items_added_by")
                            .from(QueueItems::Table, QueueItems::AddedBy)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(QueueItems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum QueueItems {
    Table,
    Id,
    RoomId,
    TrackId,
    Provider,
    Title,
    Artist,
    Uri,
    AddedBy,
    Priority,
    Votes,
    PlayedAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Rooms {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
