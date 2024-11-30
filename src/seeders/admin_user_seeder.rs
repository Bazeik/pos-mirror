use sea_orm::{DatabaseConnection, EntityTrait, ActiveValue::Set};
use crate::entities::prelude::{AdminUser, AdminUserActiveModel};
use crate::utils::helpers::password_hash::hash_password;
use chrono::Utc;

/// Seeds the user table with initial data
pub async fn seed(db: &DatabaseConnection) -> Result<(), sea_orm::DbErr> {
    // Clean the table before seeding
    println!("Cleaning user table...");
    AdminUser::delete_many().exec(db).await?;
    println!("Users table cleaned!");

    println!("Seeding user table...");

    // Current timestamp for created_at and updated_at
    let now = Utc::now().naive_utc();

    // Sample user data
    let admin_user_data = vec![
        AdminUserActiveModel {
            username: Set("admin".to_owned()),
            password: Set(hash_password("password123")),
            email: Set("admin@example.com".to_owned()),
            full_name: Set("Admin User".to_owned()),
            created_at: Set(now),
            updated_at: Set(now),
            ..Default::default()
        },
    ];

    // Insert users into the database
    AdminUser::insert_many(admin_user_data).exec(db).await?;

    println!("Admin Users table seeded successfully!");
    Ok(())
}