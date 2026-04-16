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
                    .col(ColumnDef::new(Votes::Id).string().not_null().primary_key())
                    .col(ColumnDef::new(Votes::QueueItemId).string().not_null())
                    .col(ColumnDef::new(Votes::UserId).string().not_null())
                    .col(
                        ColumnDef::new(Votes::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_votes_queue_item_id")
                            .from(Votes::Table, Votes::QueueItemId)
                            .to(QueueItems::Table, QueueItems::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_votes_user_id")
                            .from(Votes::Table, Votes::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_votes_queue_item_id_user_id")
                    .table(Votes::Table)
                    .col(Votes::QueueItemId)
                    .col(Votes::UserId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_votes_queue_item_id_user_id")
                    .table(Votes::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Votes::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Votes {
    Table,
    Id,
    QueueItemId,
    UserId,
    CreatedAt,
}

#[derive(Iden)]
enum QueueItems {
    Table,
    Id,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
