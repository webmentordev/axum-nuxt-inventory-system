use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "barcode_type", rename_all = "lowercase")]
pub enum BarcodeType {
    Code128,
    Ean13,
    #[sqlx(rename = "upc_a")]
    UpcA,
    Qr,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Barcode {
    pub id: Uuid,
    pub product_id: Option<Uuid>,
    pub code: String,
    #[sqlx(rename = "type")]
    pub barcode_type: BarcodeType,
    pub is_sold: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBarcode {
    pub product_id: Option<Uuid>,
    pub barcode_type: Option<BarcodeType>,
    // code + is_sold are set server-side, not from the client
}

// Generate many at once — e.g. { "count": 50 } for blank labels,
// or { "product_id": "...", "count": 5 } for one product
#[derive(Debug, Deserialize)]
pub struct CreateBarcodesBulk {
    pub product_id: Option<Uuid>,
    pub barcode_type: Option<BarcodeType>,
    pub count: u32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBarcode {
    pub product_id: Option<Uuid>,
    pub is_sold: Option<bool>,
}

pub async fn get_barcodes() {
    println!("barcode mod!");
}

pub async fn create_barcode() {
    println!("barcode mod!");
}

pub async fn get_barcode() {
    println!("barcode mod!");
}

pub async fn update_barcode() {
    println!("barcode mod!");
}

pub async fn delete_barcode() {
    println!("barcode mod!");
}
