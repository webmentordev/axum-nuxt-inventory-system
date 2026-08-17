use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: Uuid,
    pub category_id: Uuid,
    pub sub_category_id: Option<Uuid>,

    pub name: String,
    pub sku: String,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub description: Option<String>,

    pub power_rating_watts: Option<Decimal>,
    pub voltage_rating: Option<Decimal>,
    pub capacity_ah: Option<Decimal>,
    pub warranty_months: Option<i16>,

    pub cost_price: Decimal,
    pub selling_price: Decimal,

    pub quantity_in_stock: i32,
    pub reorder_level: i32,
    pub unit: String,

    pub image_url: Option<String>,

    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub category_id: Uuid,
    pub sub_category_id: Option<Uuid>,
    pub name: String,
    pub sku: String,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub description: Option<String>,
    pub power_rating_watts: Option<Decimal>,
    pub voltage_rating: Option<Decimal>,
    pub capacity_ah: Option<Decimal>,
    pub warranty_months: Option<i16>,
    pub cost_price: Decimal,
    pub selling_price: Decimal,
    pub quantity_in_stock: Option<i32>,
    pub reorder_level: Option<i32>,
    pub unit: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub category_id: Option<Uuid>,
    pub sub_category_id: Option<Uuid>,
    pub name: Option<String>,
    pub sku: Option<String>,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub description: Option<String>,
    pub power_rating_watts: Option<Decimal>,
    pub voltage_rating: Option<Decimal>,
    pub capacity_ah: Option<Decimal>,
    pub warranty_months: Option<i16>,
    pub cost_price: Option<Decimal>,
    pub selling_price: Option<Decimal>,
    pub quantity_in_stock: Option<i32>,
    pub reorder_level: Option<i32>,
    pub unit: Option<String>,
    pub image_url: Option<String>,
    pub is_active: Option<bool>,
}

pub async fn get_products() {
    println!("products mod!");
}

pub async fn create_product() {
    println!("products mod!");
}

pub async fn get_product() {
    println!("products mod!");
}

pub async fn update_product() {
    println!("products mod!");
}

pub async fn delete_product() {
    println!("products mod!");
}
