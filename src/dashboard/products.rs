use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dashboard::products_seo::ProductSeo;
use crate::{AppState, utils::slugify};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Product {
    pub id: Uuid,
    pub category_id: Option<Uuid>,
    pub sub_category_id: Option<Uuid>,
    pub brand_id: Option<Uuid>,

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
    pub sub_category_id: Uuid,
    pub brand_id: Uuid,
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
    pub brand_id: Option<Uuid>,
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

#[derive(Debug, Clone, Serialize)]
pub struct ProductWithSeo {
    pub id: Uuid,
    pub category_id: Option<Uuid>,
    pub sub_category_id: Option<Uuid>,
    pub brand_id: Option<Uuid>,

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

    pub seo: Option<ProductSeo>,
}

impl ProductWithSeo {
    fn from_product(p: Product, seo: Option<ProductSeo>) -> Self {
        Self {
            id: p.id,
            category_id: p.category_id,
            sub_category_id: p.sub_category_id,
            brand_id: p.brand_id,
            name: p.name,
            slug: p.slug,
            sku: p.sku,
            brand: p.brand,
            model: p.model,
            description: p.description,
            power_rating_watts: p.power_rating_watts,
            voltage_rating: p.voltage_rating,
            capacity_ah: p.capacity_ah,
            warranty_months: p.warranty_months,
            cost_price: p.cost_price,
            selling_price: p.selling_price,
            quantity_in_stock: p.quantity_in_stock,
            reorder_level: p.reorder_level,
            unit: p.unit,
            image_url: p.image_url,
            is_active: p.is_active,
            created_at: p.created_at,
            updated_at: p.updated_at,
            seo,
        }
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ProductOption {
    pub id: Uuid,
    pub name: String,
    pub is_active: bool,
}

pub async fn get_products_list(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductOption>>, StatusCode> {
    let products = sqlx::query_as!(
        ProductOption,
        r#"SELECT id, name, is_active
           FROM products
           ORDER BY name ASC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(products))
}

pub async fn get_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductWithSeo>>, StatusCode> {
    let products = sqlx::query_as!(
        Product,
        r#"SELECT id, category_id, sub_category_id, brand_id, name, slug, sku, brand, model, description,
                  power_rating_watts, voltage_rating, capacity_ah, warranty_months,
                  cost_price, selling_price, quantity_in_stock, reorder_level, unit,
                  image_url, is_active, created_at, updated_at
           FROM products
           ORDER BY created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let product_ids: Vec<Uuid> = products.iter().map(|p| p.id).collect();

    let seo_rows = sqlx::query_as!(
        ProductSeo,
        r#"SELECT id, product_id, meta_title, meta_description, meta_keywords,
                  og_title, og_description, og_image_url, canonical_url, focus_keyword,
                  created_at, updated_at
           FROM products_seo
           WHERE product_id = ANY($1)"#,
        &product_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = products
        .into_iter()
        .map(|p| {
            let seo = seo_rows.iter().find(|s| s.product_id == p.id).cloned();
            ProductWithSeo::from_product(p, seo)
        })
        .collect();

    Ok(Json(result))
}

pub async fn get_product(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<ProductWithSeo>, StatusCode> {
    let product = sqlx::query_as!(
        Product,
        r#"SELECT id, category_id, sub_category_id, brand_id, name, slug, sku, brand, model, description,
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

    let seo = sqlx::query_as!(
        ProductSeo,
        r#"SELECT id, product_id, meta_title, meta_description, meta_keywords,
                  og_title, og_description, og_image_url, canonical_url, focus_keyword,
                  created_at, updated_at
           FROM products_seo
           WHERE product_id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ProductWithSeo::from_product(product, seo)))
}

pub async fn create_product(
    State(state): State<AppState>,
    Json(payload): Json<CreateProduct>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    let quantity_in_stock = payload.quantity_in_stock.unwrap_or(0);
    let reorder_level = payload.reorder_level.unwrap_or(0);
    let unit = payload.unit.unwrap_or_else(|| "piece".to_string());
    let slug = slugify(&payload.name, true);

    let product = sqlx::query_as!(
        Product,
        r#"INSERT INTO products (
               category_id, sub_category_id, brand_id, name, slug, sku, brand, model, description,
               power_rating_watts, voltage_rating, capacity_ah, warranty_months,
               cost_price, selling_price, quantity_in_stock, reorder_level, unit, image_url
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
           RETURNING id, category_id, sub_category_id, brand_id, name, slug, sku, brand, model, description,
                     power_rating_watts, voltage_rating, capacity_ah, warranty_months,
                     cost_price, selling_price, quantity_in_stock, reorder_level, unit,
                     image_url, is_active, created_at, updated_at"#,
        Some(payload.category_id),
        Some(payload.sub_category_id),
        Some(payload.brand_id),
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

pub async fn update_product(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateProduct>,
) -> Result<Json<Product>, StatusCode> {
    let new_slug = payload.name.as_ref().map(|name| slugify(name, true));

    let product = sqlx::query_as!(
        Product,
        r#"UPDATE products
           SET category_id = COALESCE($1, category_id),
               sub_category_id = COALESCE($2, sub_category_id),
               brand_id = COALESCE($3, brand_id),
               name = COALESCE($4, name),
               slug = COALESCE($5, slug),
               sku = COALESCE($6, sku),
               brand = COALESCE($7, brand),
               model = COALESCE($8, model),
               description = COALESCE($9, description),
               power_rating_watts = COALESCE($10, power_rating_watts),
               voltage_rating = COALESCE($11, voltage_rating),
               capacity_ah = COALESCE($12, capacity_ah),
               warranty_months = COALESCE($13, warranty_months),
               cost_price = COALESCE($14, cost_price),
               selling_price = COALESCE($15, selling_price),
               quantity_in_stock = COALESCE($16, quantity_in_stock),
               reorder_level = COALESCE($17, reorder_level),
               unit = COALESCE($18, unit),
               image_url = COALESCE($19, image_url),
               is_active = COALESCE($20, is_active),
               updated_at = NOW()
           WHERE id = $21
           RETURNING id, category_id, sub_category_id, brand_id, name, slug, sku, brand, model, description,
                     power_rating_watts, voltage_rating, capacity_ah, warranty_months,
                     cost_price, selling_price, quantity_in_stock, reorder_level, unit,
                     image_url, is_active, created_at, updated_at"#,
        payload.category_id,
        payload.sub_category_id,
        payload.brand_id,
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
