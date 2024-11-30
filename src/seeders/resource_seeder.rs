// src/seeders/seed_resources.rs
use sea_orm::{entity::*, DatabaseConnection, DbErr};
use sea_orm::prelude::*;
use serde_json::Value;
use std::fs;

pub async fn seed(db: &DatabaseConnection) -> Result<(), DbErr> {
    // Read the resources from the JSON file
    let data = fs::read_to_string("src/seeders/seeds/resources.json")
        .expect("Unable to read file");
    let resources: Vec<Value> = serde_json::from_str(&data)
        .expect("JSON was not well-formatted");

    // Insert each resource into the database
    for resource in resources {
        let id = resource["id"].as_i64().unwrap() as i32;
        let name = resource["name"].as_str().unwrap().to_string();
        let slug = resource["slug"].as_str().unwrap().to_string();

        let resource_data = ActiveModel {
            id: Set(id),
            name: Set(name),
            slug: Set(slug),
            ..Default::default()
        };

        resource_data.insert(db).await?;
    }

    println!("Resources seeded successfully!");
    Ok(())
}