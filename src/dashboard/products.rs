use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

use crate::auth::Claims;
use crate::dashboard::products_seo::ProductSeo;
use crate::{AppState, utils::*};

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
    pub content: Option<String>,

    pub warranty_months: Option<i16>,

    pub cost_price: Decimal,
    pub selling_price: Decimal,
    pub compare_at_selling_price: Option<Decimal>,
    pub shipping_cost: Decimal,
    pub tax: Decimal,

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
    pub brand: Option<String>,
    pub model: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub warranty_months: Option<i16>,
    pub cost_price: Decimal,
    pub selling_price: Decimal,
    pub compare_at_selling_price: Option<Decimal>,
    pub shipping_cost: Option<Decimal>,
    pub tax: Option<Decimal>,
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
    pub brand: Option<String>,
    pub model: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub warranty_months: Option<i16>,
    pub cost_price: Option<Decimal>,
    pub selling_price: Option<Decimal>,
    pub compare_at_selling_price: Option<Decimal>,
    pub shipping_cost: Option<Decimal>,
    pub tax: Option<Decimal>,
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
    pub content: Option<String>,

    pub warranty_months: Option<i16>,

    pub cost_price: Decimal,
    pub selling_price: Decimal,
    pub compare_at_selling_price: Option<Decimal>,
    pub shipping_cost: Decimal,
    pub tax: Decimal,

    pub quantity_in_stock: i32,
    pub reorder_level: i32,
    pub unit: String,

    pub image_url: Option<String>,

    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    pub seo: Option<ProductSeo>,
    pub total_specs_count: i64,
}

impl ProductWithSeo {
    fn from_product(p: Product, seo: Option<ProductSeo>, total_specs_count: i64) -> Self {
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
            content: p.content,
            warranty_months: p.warranty_months,
            cost_price: p.cost_price,
            selling_price: p.selling_price,
            compare_at_selling_price: p.compare_at_selling_price,
            shipping_cost: p.shipping_cost,
            tax: p.tax,
            quantity_in_stock: p.quantity_in_stock,
            reorder_level: p.reorder_level,
            unit: p.unit,
            image_url: p.image_url,
            is_active: p.is_active,
            created_at: p.created_at,
            updated_at: p.updated_at,
            seo,
            total_specs_count,
        }
    }
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct ProductOption {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub sku: String,
    pub is_active: bool,
    pub quantity_in_stock: i32,
}

pub async fn get_products_list(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductOption>>, StatusCode> {
    let products = sqlx::query_as!(
        ProductOption,
        r#"SELECT id, name, slug, sku, is_active, quantity_in_stock
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
        r#"SELECT id, category_id, sub_category_id, brand_id, name, slug, sku, brand, model, description, content,
                  warranty_months,
                  cost_price, selling_price, compare_at_selling_price,
                  shipping_cost, tax,
                  quantity_in_stock, reorder_level, unit,
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

    let spec_counts: HashMap<Uuid, i64> = sqlx::query!(
        r#"SELECT product_id, COUNT(*) AS "count!"
           FROM product_specifications
           WHERE product_id = ANY($1)
           GROUP BY product_id"#,
        &product_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(|r| (r.product_id, r.count))
    .collect();

    let result = products
        .into_iter()
        .map(|p| {
            let seo = seo_rows.iter().find(|s| s.product_id == p.id).cloned();
            let count = spec_counts.get(&p.id).copied().unwrap_or(0);
            ProductWithSeo::from_product(p, seo, count)
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
        r#"SELECT id, category_id, sub_category_id, brand_id, name, slug, sku, brand, model, description, content,
                  warranty_months,
                  cost_price, selling_price, compare_at_selling_price,
                  shipping_cost, tax,
                  quantity_in_stock, reorder_level, unit,
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

    let total_specs_count = sqlx::query_scalar!(
        r#"SELECT COUNT(*) AS "count!" FROM product_specifications WHERE product_id = $1"#,
        uuid
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ProductWithSeo::from_product(
        product,
        seo,
        total_specs_count,
    )))
}

pub async fn create_product(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateProduct>,
) -> Result<(StatusCode, Json<Product>), StatusCode> {
    let quantity_in_stock = payload.quantity_in_stock.unwrap_or(0);
    let reorder_level = payload.reorder_level.unwrap_or(0);
    let unit = payload.unit.unwrap_or_else(|| "piece".to_string());
    let shipping_cost = payload.shipping_cost.unwrap_or_default();
    let tax = payload.tax.unwrap_or_default();
    let slug = slugify(&payload.name, true);

    let product = sqlx::query_as!(
        Product,
        r#"INSERT INTO products (
               category_id, sub_category_id, brand_id, name, slug, sku, brand, model, description, content,
               warranty_months,
               cost_price, selling_price, compare_at_selling_price,
               shipping_cost, tax,
               quantity_in_stock, reorder_level, unit, image_url
           )
           VALUES (
               $1, $2, $3, $4, $5, $6, $7, $8, $9, $10,
               $11,
               $12, $13, $14,
               $15, $16,
               $17, $18, $19, $20
           )
           RETURNING id, category_id, sub_category_id, brand_id, name, slug, sku, brand, model, description, content,
                     warranty_months,
                     cost_price, selling_price, compare_at_selling_price,
                     shipping_cost, tax,
                     quantity_in_stock, reorder_level, unit,
                     image_url, is_active, created_at, updated_at"#,
        Some(payload.category_id),
        Some(payload.sub_category_id),
        Some(payload.brand_id),
        payload.name,
        slug,
        generate_sku(&payload.name),
        payload.brand,
        payload.model,
        payload.description,
        payload.content,
        payload.warranty_months,
        payload.cost_price,
        payload.selling_price,
        payload.compare_at_selling_price,
        shipping_cost,
        tax,
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

    log_audit(
        &state.db,
        Some(claims.sub),
        "create",
        "product",
        Some(product.id),
        "created",
        Some(json!({ "name": product.name, "slug": product.slug, "sku": product.sku })),
    )
    .await;

    Ok((StatusCode::CREATED, Json(product)))
}

pub async fn update_product(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateProduct>,
) -> Result<Json<Product>, StatusCode> {
    let new_slug = payload.name.as_ref().map(|name| slugify(name, true));
    let product = sqlx::query_as!(
        Product,
        r#"UPDATE products
           SET category_id = COALESCE($1, category_id),
               name = COALESCE($2, name),
               slug = COALESCE($3, slug),
               brand = COALESCE($4, brand),
               cost_price = COALESCE($5, cost_price),
               selling_price = COALESCE($6, selling_price),
               shipping_cost = COALESCE($7, shipping_cost),
               tax = COALESCE($8, tax),
               quantity_in_stock = COALESCE($9, quantity_in_stock),
               reorder_level = COALESCE($10, reorder_level),
               unit = COALESCE($11, unit),
               is_active = COALESCE($12, is_active),
               sub_category_id = $13,
               brand_id = $14,
               model = $15,
               description = $16,
               content = $17,
               warranty_months = $18,
               compare_at_selling_price = $19,
               image_url = $20,
               updated_at = NOW()
           WHERE id = $21
           RETURNING id, category_id, sub_category_id, brand_id, name, slug, sku, brand, model, description, content,
                     warranty_months,
                     cost_price, selling_price, compare_at_selling_price,
                     shipping_cost, tax,
                     quantity_in_stock, reorder_level, unit,
                     image_url, is_active, created_at, updated_at"#,
        payload.category_id,
        payload.name,
        new_slug,
        payload.brand,
        payload.cost_price,
        payload.selling_price,
        payload.shipping_cost,
        payload.tax,
        payload.quantity_in_stock,
        payload.reorder_level,
        payload.unit,
        payload.is_active,
        payload.sub_category_id,
        payload.brand_id,
        payload.model,
        payload.description,
        payload.content,
        payload.warranty_months,
        payload.compare_at_selling_price,
        payload.image_url,
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

    log_audit(
        &state.db,
        Some(claims.sub),
        "update",
        "product",
        Some(product.id),
        "updated",
        Some(json!({ "name": product.name, "slug": product.slug, "is_active": product.is_active })),
    )
    .await;

    Ok(Json(product))
}
