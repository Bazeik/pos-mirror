use sea_orm_migration::prelude::*;
use crate::migration::{User, Permission, UserPermission, FK_USERPERMISSION_USER, FK_USERPERMISSION_PERMISSION};

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
                            .name(FK_USERPERMISSION_USER)
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
                            .name(FK_USERPERMISSION_PERMISSION) // Use the constant for the FK name
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
                    .drop_foreign_key(Alias::new(FK_USERPERMISSION_USER)) // Use the constant wrapped in `Alias`
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(UserPermission::Table)
                    .drop_foreign_key(Alias::new(FK_USERPERMISSION_PERMISSION)) // Use the constant wrapped in `Alias`
                    .to_owned(),
            )
            .await?;

        // Drop the table
        manager
            .drop_table(
                Table::drop()
                    .table(UserPermission::Table)
                    .to_owned(),
            )
            .await
    }
}