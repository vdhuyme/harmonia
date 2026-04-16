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
                    .col(ColumnDef::new(QueueItems::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(QueueItems::RoomId).string().not_null())
                    .col(ColumnDef::new(QueueItems::Provider).string().not_null())
                    .col(ColumnDef::new(QueueItems::TrackId).string().not_null())
                    .col(ColumnDef::new(QueueItems::Title).string().not_null())
                    .col(ColumnDef::new(QueueItems::Artist).string().not_null())
                    .col(ColumnDef::new(QueueItems::Priority).integer().not_null().default(0))
                    .col(ColumnDef::new(QueueItems::Votes).integer().not_null().default(0))
                    .col(
                        ColumnDef::new(QueueItems::Status)
                            .string()
                            .not_null()
                            .default("pending"),
                    )
                    .col(
                        ColumnDef::new(QueueItems::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(QueueItems::UpdatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(QueueItems::StartedAt).timestamp())
                    .col(ColumnDef::new(QueueItems::EndedAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_queue_items_room_id")
                            .from(QueueItems::Table, QueueItems::RoomId)
                            .to(Rooms::Table, Rooms::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_queue_items_room_id")
                    .table(QueueItems::Table)
                    .col(QueueItems::RoomId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_queue_items_status")
                    .table(QueueItems::Table)
                    .col(QueueItems::Status)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_queue_items_status")
                    .table(QueueItems::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_index(
                Index::drop()
                    .name("idx_queue_items_room_id")
                    .table(QueueItems::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(QueueItems::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum QueueItems {
    Table,
    Id,
    RoomId,
    Provider,
    TrackId,
    Title,
    Artist,
    Priority,
    Votes,
    Status,
    CreatedAt,
    UpdatedAt,
    StartedAt,
    EndedAt,
}

#[derive(Iden)]
enum Rooms {
    Table,
    Id,
}
