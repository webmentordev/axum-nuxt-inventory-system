use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SubCategory {
    pub id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubCategory {
    pub category_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubCategory {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

pub async fn get_sub_categories() {
    println!("sub_categories mod!");
}

pub async fn create_sub_category() {
    println!("sub_categories mod!");
}

pub async fn get_sub_category() {
    println!("sub_categories mod!");
}

pub async fn update_sub_category() {
    println!("sub_categories mod!");
}

pub async fn delete_sub_category() {
    println!("sub_categories mod!");
}
