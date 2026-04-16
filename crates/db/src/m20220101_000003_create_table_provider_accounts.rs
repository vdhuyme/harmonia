use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProviderAccounts::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ProviderAccounts::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(ProviderAccounts::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProviderAccounts::Provider)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProviderAccounts::ProviderUserId)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProviderAccounts::AccessToken)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProviderAccounts::RefreshToken)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProviderAccounts::ExpiresAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ProviderAccounts::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(ProviderAccounts::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_provider_accounts_user_id")
                            .from(
                                ProviderAccounts::Table,
                                ProviderAccounts::UserId,
                            )
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ProviderAccounts::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ProviderAccounts {
    Table,
    Id,
    UserId,
    Provider,
    ProviderUserId,
    AccessToken,
    RefreshToken,
    ExpiresAt,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
