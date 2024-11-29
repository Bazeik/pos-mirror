use sea_orm_migration::prelude::*;
use crate::migration::Resource;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Resource::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Resource::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Resource::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(Resource::Slug)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Resource::Table)
                    .to_owned(),
            )
            .await
    }
}