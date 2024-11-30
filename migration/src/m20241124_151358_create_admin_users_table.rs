use sea_orm_migration::prelude::*;
use crate::migration::AdminUser;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminUser::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AdminUser::Username)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(AdminUser::Password)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AdminUser::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(AdminUser::FullName)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AdminUser::CreatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AdminUser::UpdatedAt)
                            .date_time()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AdminUser::Table).to_owned())
            .await
    }
}