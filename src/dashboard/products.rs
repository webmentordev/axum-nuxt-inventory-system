use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: Uuid,
    pub category_id: Uuid,
    pub sub_category_id: Option<Uuid>,

    pub name: String,
    pub slug: String,
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

fn slugify(name: &str) -> String {
    let base: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect();

    let base: Vec<&str> = base.split('-').filter(|s| !s.is_empty()).collect();
    let base = base.join("-");

    let suffix: u32 = rand::random_range(1000..10000);

    format!("{base}-{suffix}")
}

pub async fn get_products(State(state): State<AppState>) -> Result<Json<Vec<Product>>, StatusCode> {
    let products = sqlx::query_as!(
        Product,
        r#"SELECT id, category_id, sub_category_id, name, slug, sku, brand, model, description,
                  power_rating_watts, voltage_rating, capacity_ah, warranty_months,
                  cost_price, selling_price, quantity_in_stock, reorder_level, unit,
                  image_url, is_active, created_at, updated_at
           FROM products
           ORDER BY created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(products))
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProduct>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    let quantity_in_stock = payload.quantity_in_stock.unwrap_or(0);
    let reorder_level = payload.reorder_level.unwrap_or(0);
    let unit = payload.unit.unwrap_or_else(|| "piece".to_string());
    let slug = slugify(&payload.name);

    let product = sqlx::query_as!(
        Product,
        r#"INSERT INTO products (
               category_id, sub_category_id, name, slug, sku, brand, model, description,
               power_rating_watts, voltage_rating, capacity_ah, warranty_months,
               cost_price, selling_price, quantity_in_stock, reorder_level, unit, image_url
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
           RETURNING id, category_id, sub_category_id, name, slug, sku, brand, model, description,
                     power_rating_watts, voltage_rating, capacity_ah, warranty_months,
                     cost_price, selling_price, quantity_in_stock, reorder_level, unit,
                     image_url, is_active, created_at, updated_at"#,
        payload.category_id,
        payload.sub_category_id,
        payload.name,
        slug,
        payload.sku,
        payload.brand,
        payload.model,
        payload.description,
        payload.power_rating_watts,
        payload.voltage_rating,
        payload.capacity_ah,
        payload.warranty_months,
        payload.cost_price,
        payload.selling_price,
        quantity_in_stock,
        reorder_level,
        unit,
        payload.image_url
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23503") => {
            StatusCode::BAD_REQUEST
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::CREATED, Json(product)))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<Product>, StatusCode> {
    let product = sqlx::query_as!(
        Product,
        r#"SELECT id, category_id, sub_category_id, name, slug, sku, brand, model, description,
                  power_rating_watts, voltage_rating, capacity_ah, warranty_months,
                  cost_price, selling_price, quantity_in_stock, reorder_level, unit,
                  image_url, is_active, created_at, updated_at
           FROM products
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(product))
}

pub async fn update_product(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateProduct>,
) -> Result<Json<Product>, StatusCode> {
    let new_slug = payload.name.as_ref().map(|name| slugify(name));

    let product = sqlx::query_as!(
        Product,
        r#"UPDATE products
           SET category_id = COALESCE($1, category_id),
               sub_category_id = COALESCE($2, sub_category_id),
               name = COALESCE($3, name),
               slug = COALESCE($4, slug),
               sku = COALESCE($5, sku),
               brand = COALESCE($6, brand),
               model = COALESCE($7, model),
               description = COALESCE($8, description),
               power_rating_watts = COALESCE($9, power_rating_watts),
               voltage_rating = COALESCE($10, voltage_rating),
               capacity_ah = COALESCE($11, capacity_ah),
               warranty_months = COALESCE($12, warranty_months),
               cost_price = COALESCE($13, cost_price),
               selling_price = COALESCE($14, selling_price),
               quantity_in_stock = COALESCE($15, quantity_in_stock),
               reorder_level = COALESCE($16, reorder_level),
               unit = COALESCE($17, unit),
               image_url = COALESCE($18, image_url),
               is_active = COALESCE($19, is_active),
               updated_at = NOW()
           WHERE id = $20
           RETURNING id, category_id, sub_category_id, name, slug, sku, brand, model, description,
                     power_rating_watts, voltage_rating, capacity_ah, warranty_months,
                     cost_price, selling_price, quantity_in_stock, reorder_level, unit,
                     image_url, is_active, created_at, updated_at"#,
        payload.category_id,
        payload.sub_category_id,
        payload.name,
        new_slug,
        payload.sku,
        payload.brand,
        payload.model,
        payload.description,
        payload.power_rating_watts,
        payload.voltage_rating,
        payload.capacity_ah,
        payload.warranty_months,
        payload.cost_price,
        payload.selling_price,
        payload.quantity_in_stock,
        payload.reorder_level,
        payload.unit,
        payload.image_url,
        payload.is_active,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23503") => {
            StatusCode::BAD_REQUEST
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(product))
}

pub async fn delete_product(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM products WHERE id = $1", uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
