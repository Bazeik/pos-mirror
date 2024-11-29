use sea_orm_migration::prelude::*;
use crate::migration::{RolePermission, Role, Permission};

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
                        .name(RolePermission::FkRole)
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
                        .name(RolePermission::FkPermission)
                        .from(RolePermission::Table, RolePermission::PermissionId)
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
                    .table(RolePermission::Table)
                    .drop_foreign_key(RolePermission::FkRole)
                    .to_owned(),
            )
            .await?;
        
        manager
            .alter_table(
                Table::alter()
                    .table(RolePermission::Table)
                    .drop_foreign_key(RolePermission::FkPermission)
                    .to_owned(),
            )
            .await?;
        
        manager
            .drop_table(
                Table::drop()
                    .table(RolePermission::Table)
                    .to_owned(),
            )
            .await
    }
}