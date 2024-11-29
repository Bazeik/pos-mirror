use sea_orm_migration::prelude::*;
use crate::migration::{Permission, Resource, FK_PERMISSION_RESOURCE};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Permission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Permission::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Permission::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Permission::Slug)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Permission::ResourceId)
                            .integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name(FK_PERMISSION_RESOURCE) // Use the constant from lib.rs
                            .from(Permission::Table, Permission::ResourceId)
                            .to(Resource::Table, Resource::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the foreign key before dropping the table
        manager
            .alter_table(
                Table::alter()
                    .table(Permission::Table)
                    .drop_foreign_key(Alias::new(FK_PERMISSION_RESOURCE)) // Use the constant from lib.rs
                    .to_owned(),
            )
            .await?;

        // Drop the table
        manager
            .drop_table(
                Table::drop()
                    .table(Permission::Table)
                    .to_owned(),
            )
            .await
    }
}