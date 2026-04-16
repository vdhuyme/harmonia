use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Votes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Votes::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Votes::UserId).uuid().not_null())
                    .col(ColumnDef::new(Votes::QueueItemId).uuid().not_null())
                    .col(ColumnDef::new(Votes::Value).integer().not_null())
                    .col(
                        ColumnDef::new(Votes::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_votes_user_id")
                            .from(Votes::Table, Votes::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_votes_queue_item_id")
                            .from(Votes::Table, Votes::QueueItemId)
                            .to(QueueItems::Table, QueueItems::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Votes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Votes {
    Table,
    Id,
    UserId,
    QueueItemId,
    Value,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum QueueItems {
    Table,
    Id,
}
