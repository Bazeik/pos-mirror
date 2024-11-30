//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "admin_user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    #[sea_orm(unique)]
    pub email: String,
    pub full_name: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::admin_user_permission::Entity")]
    AdminUserPermission,
}

impl Related<super::admin_user_permission::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::AdminUserPermission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}