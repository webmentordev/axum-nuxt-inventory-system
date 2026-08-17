use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

pub async fn get_categories() {
    println!("categories mod!");
}

pub async fn create_category() {
    println!("categories mod!");
}

pub async fn get_category() {
    println!("categories mod!");
}

pub async fn update_category() {
    println!("categories mod!");
}

pub async fn delete_category() {
    println!("categories mod!");
}
