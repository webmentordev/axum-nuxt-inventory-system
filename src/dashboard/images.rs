use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Image {
    pub id: Uuid,
    pub product_id: Option<Uuid>,
    pub name: String,
    pub file_path: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateImage {
    pub product_id: Option<Uuid>,
    pub name: String,
    // file_path is set by the handler after saving the uploaded file,
    // not taken from the client
}

pub async fn get_images() {
    println!("Image mod!");
}

pub async fn create_image() {
    println!("Image mod!");
}

pub async fn get_image() {
    println!("Image mod!");
}

pub async fn update_image() {
    println!("Image mod!");
}

pub async fn delete_image() {
    println!("Image mod!");
}
