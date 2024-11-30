use sea_orm_migration::prelude::*;
use crate::migration::{AdminUser, Permission, AdminUserPermission, FK_ADMINUSERPERMISSION_ADMINUSER, FK_ADMINUSERPERMISSION_PERMISSION};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminUserPermission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminUserPermission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AdminUserPermission::AdminUserId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_ADMINUSERPERMISSION_ADMINUSER)
                            .from(AdminUserPermission::Table, AdminUserPermission::AdminUserId)
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(AdminUserPermission::PermissionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_ADMINUSERPERMISSION_PERMISSION) // Use the constant for the FK name
                            .from(AdminUserPermission::Table, AdminUserPermission::PermissionId)
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
                    .table(AdminUserPermission::Table)
                    .drop_foreign_key(Alias::new(FK_ADMINUSERPERMISSION_ADMINUSER)) // Use the constant wrapped in `Alias`
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserPermission::Table)
                    .drop_foreign_key(Alias::new(FK_ADMINUSERPERMISSION_PERMISSION)) // Use the constant wrapped in `Alias`
                    .to_owned(),
            )
            .await?;

        // Drop the table
        manager
            .drop_table(
                Table::drop()
                    .table(AdminUserPermission::Table)
                    .to_owned(),
            )
            .await
    }
}