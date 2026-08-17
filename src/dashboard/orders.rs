use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "order_status", rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Walkin,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Order {
    pub id: Uuid,
    pub order_number: String,

    pub customer_name: String,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    pub shipping_address: Option<String>,

    pub status: OrderStatus,

    pub subtotal: Decimal,
    pub tax_amount: Decimal,
    pub shipping_amount: Decimal,
    pub total_amount: Decimal,

    pub notes: Option<String>,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,

    pub product_name: String,
    pub product_sku: String,
    pub unit_price: Decimal,
    pub quantity: i32,
    pub line_total: Decimal,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrderItem {
    pub product_id: Uuid,
    pub quantity: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateOrder {
    pub customer_name: String,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    pub shipping_address: Option<String>,
    pub notes: Option<String>,
    pub items: Vec<CreateOrderItem>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateOrder {
    pub customer_name: Option<String>,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    pub shipping_address: Option<String>,
    pub status: Option<OrderStatus>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct OrderWithItems {
    #[serde(flatten)]
    pub order: Order,
    pub items: Vec<OrderItem>,
}
pub async fn get_orders() {
    println!("orders mod!");
}

pub async fn create_order() {
    println!("orders mod!");
}

pub async fn get_order() {
    println!("orders mod!");
}

pub async fn update_order() {
    println!("orders mod!");
}

pub async fn delete_order() {
    println!("orders mod!");
}
