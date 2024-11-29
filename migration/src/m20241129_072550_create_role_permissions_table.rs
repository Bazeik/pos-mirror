use sea_orm_migration::prelude::*;
use crate::migration::{RolePermission, Role, Permission, FK_ROLEPERMISSION_ROLE, FK_ROLEPERMISSION_PERMISSION};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RolePermission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(RolePermission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(RolePermission::RoleId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_ROLEPERMISSION_ROLE)
                            .from(RolePermission::Table, RolePermission::RoleId)
                            .to(Role::Table, Role::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(RolePermission::PermissionId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_ROLEPERMISSION_PERMISSION)
                            .from(RolePermission::Table, RolePermission::PermissionId)
                            .to(Permission::Table, Permission::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop foreign keys explicitly before dropping the table
        manager
            .alter_table(
                Table::alter()
                    .table(RolePermission::Table)
                    .drop_foreign_key(Alias::new(FK_ROLEPERMISSION_ROLE))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(RolePermission::Table)
                    .drop_foreign_key(Alias::new(FK_ROLEPERMISSION_PERMISSION))
                    .to_owned(),
            )
            .await?;

        // Drop the table
        manager
            .drop_table(
                Table::drop()
                    .table(RolePermission::Table)
                    .to_owned(),
            )
            .await
    }
}