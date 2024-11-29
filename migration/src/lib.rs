pub use sea_orm_migration::prelude::*;

pub struct Migrator;

mod m20241124_151359_create_users_table;
mod m20241128_120615_create_resources_table;
mod m20241128_121404_create_roles_table;
mod m20241128_121428_create_permissions_table;
mod m20241129_072550_create_role_permissions_table;
mod m20241129_072555_create_user_permissions_table;

// Enum Definitions for Table and Column Names
#[derive(Iden)]
pub enum User {
    Table,
    Id,
    Username,
    Password,
    Email,
    FullName,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden)]
pub enum Resource {
    Table,
    Id,
    Name,
    Slug,
}

#[derive(Iden)]
pub enum Role {
    Table,
    Id,
    Name,
    Slug,
}

#[derive(Iden)]
pub enum Permission {
    Table,
    Id,
    Name,
    Slug,
    ResourceId,
}

#[derive(Iden)]
pub enum RolePermission {
    Table,
    Id,
    RoleId,
    PermissionId,
}

#[derive(Iden)]
pub enum UserPermission {
    Table,
    Id,
    UserId,
    PermissionId,
}

// Foreign Key Names as Constants
pub const FK_PERMISSION_RESOURCE: &str = "fk_permission_resource";
pub const FK_ROLEPERMISSION_ROLE: &str = "fk_rolepermission_role";
pub const FK_ROLEPERMISSION_PERMISSION: &str = "fk_rolepermission_permission";
pub const FK_USERPERMISSION_USER: &str = "fk_userpermission_user";
pub const FK_USERPERMISSION_PERMISSION: &str = "fk_userpermission_permission";

// Migrator Implementation
#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241124_151359_create_users_table::Migration),
            Box::new(m20241128_120615_create_resources_table::Migration),
            Box::new(m20241128_121404_create_roles_table::Migration),
            Box::new(m20241128_121428_create_permissions_table::Migration),
            Box::new(m20241129_072550_create_role_permissions_table::Migration),
            Box::new(m20241129_072555_create_user_permissions_table::Migration),
        ]
    }
}