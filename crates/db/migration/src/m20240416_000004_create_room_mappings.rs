use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RoomMappings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RoomMappings::Id)
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(RoomMappings::RoomId).string().not_null())
                    .col(ColumnDef::new(RoomMappings::ProviderAccountId).string().not_null())
                    .col(ColumnDef::new(RoomMappings::DeviceId).string().not_null())
                    .col(
                        ColumnDef::new(RoomMappings::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_room_mappings_room_id")
                            .from(RoomMappings::Table, RoomMappings::RoomId)
                            .to(Rooms::Table, Rooms::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_room_mappings_provider_account_id")
                            .from(RoomMappings::Table, RoomMappings::ProviderAccountId)
                            .to(ProviderAccounts::Table, ProviderAccounts::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_room_mappings_room_id")
                    .table(RoomMappings::Table)
                    .col(RoomMappings::RoomId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_room_mappings_room_id")
                    .table(RoomMappings::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(RoomMappings::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum RoomMappings {
    Table,
    Id,
    RoomId,
    ProviderAccountId,
    DeviceId,
    CreatedAt,
}

#[derive(Iden)]
enum Rooms {
    Table,
    Id,
}

#[derive(Iden)]
enum ProviderAccounts {
    Table,
    Id,
}
