use sea_orm_migration::prelude::*;
use crate::migration::{UserPermission, User, Permission};


#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                .table(UserPermission::Table)
                .if_not_exists()
                .col(
                    ColumnDef::new(UserPermission::Id)
                        .integer()
                        .not_null()
                        .auto_increment()
                        .primary_key(),
                )
                .col(
                    ColumnDef::new(UserPermission::UserId)
                        .integer()
                        .not_null(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name(UserPermission::FkUser)
                        .from(UserPermission::Table, UserPermission::UserId)
                        .to(User::Table, User::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .col(
                    ColumnDef::new(UserPermission::PermissionId)
                        .integer()
                        .not_null(),
                )
                .foreign_key(
                    ForeignKey::create()
                        .name(UserPermission::FkPermission)
                        .from(UserPermission::Table, UserPermission::PermissionId)
                        .to(Permission::Table, Permission::Id)
                        .on_delete(ForeignKeyAction::Cascade),
                )
                .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(UserPermission::Table)
                    .drop_foreign_key(UserPermission::FkUser)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserPermission::Table)
                    .drop_foreign_key(UserPermission::FkPermission)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(UserPermission::Table)
                    .to_owned(),
            )
            .await
    }
}