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
                            .string()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ProviderAccounts::Provider).string().not_null())
                    .col(ColumnDef::new(ProviderAccounts::UserId).string().not_null())
                    .col(ColumnDef::new(ProviderAccounts::AccessToken).text().not_null())
                    .col(ColumnDef::new(ProviderAccounts::RefreshToken).text())
                    .col(ColumnDef::new(ProviderAccounts::ExpiresAt).timestamp().not_null())
                    .col(
                        ColumnDef::new(ProviderAccounts::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_provider_accounts_user_id")
                            .from(ProviderAccounts::Table, ProviderAccounts::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_provider_accounts_user_id")
                    .table(ProviderAccounts::Table)
                    .col(ProviderAccounts::UserId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_provider_accounts_user_id")
                    .table(ProviderAccounts::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(ProviderAccounts::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum ProviderAccounts {
    Table,
    Id,
    Provider,
    UserId,
    AccessToken,
    RefreshToken,
    ExpiresAt,
    CreatedAt,
}

#[derive(Iden)]
enum Users {
    Table,
    Id,
}
