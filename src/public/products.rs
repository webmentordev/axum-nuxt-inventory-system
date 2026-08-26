use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    AppState,
    dashboard::images::{Image, WithFullUrl},
};

const SUGGESTED_PRODUCTS_LIMIT: i64 = 4;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicProductRow {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub sku: String,
    pub brand_id: Option<Uuid>,
    pub model: Option<String>,
    pub description: Option<String>,
    pub power_rating_watts: Option<Decimal>,
    pub voltage_rating: Option<Decimal>,
    pub capacity_ah: Option<Decimal>,
    pub warranty_months: Option<i16>,
    pub selling_price: Decimal,
    pub quantity_in_stock: i32,
    pub image_url: String,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicProductBrand {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub images: Vec<Image>,
}

#[derive(Debug, Serialize)]
pub struct PublicProduct {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub sku: String,
    pub brand: Option<PublicProductBrand>,
    pub model: Option<String>,
    pub description: Option<String>,
    pub power_rating_watts: Option<Decimal>,
    pub voltage_rating: Option<Decimal>,
    pub capacity_ah: Option<Decimal>,
    pub warranty_months: Option<i16>,
    pub selling_price: Decimal,
    pub in_stock: bool,
    pub unit: String,
    pub image_url: String,
    pub images: Vec<Image>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_products: Option<Vec<PublicProduct>>,
}

struct BrandRow {
    id: Uuid,
    name: String,
    slug: String,
}

pub async fn fetch_product_brands(
    state: &AppState,
    brand_ids: &[Uuid],
) -> Result<HashMap<Uuid, PublicProductBrand>, StatusCode> {
    let brands = sqlx::query_as!(
        BrandRow,
        r#"SELECT id, name, slug
           FROM brands
           WHERE id = ANY($1)"#,
        brand_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let images = sqlx::query_as!(
        Image,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, created_at
           FROM images
           WHERE brand_id = ANY($1)"#,
        brand_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let map = brands
        .into_iter()
        .map(|b| {
            let brand_images = images
                .iter()
                .filter(|img| img.brand_id == Some(b.id))
                .cloned()
                .map(Image::with_full_url)
                .collect();

            (
                b.id,
                PublicProductBrand {
                    id: b.id,
                    name: b.name,
                    slug: b.slug,
                    images: brand_images,
                },
            )
        })
        .collect();

    Ok(map)
}

pub fn build_public_product(
    p: PublicProductRow,
    images: &[Image],
    brand_map: &HashMap<Uuid, PublicProductBrand>,
) -> PublicProduct {
    let product_images = images
        .iter()
        .filter(|img| img.product_id == Some(p.id))
        .cloned()
        .map(Image::with_full_url)
        .collect();

    let brand = p.brand_id.and_then(|id| brand_map.get(&id)).cloned();

    PublicProduct {
        id: p.id,
        name: p.name,
        slug: p.slug,
        sku: p.sku,
        brand,
        model: p.model,
        description: p.description,
        power_rating_watts: p.power_rating_watts,
        voltage_rating: p.voltage_rating,
        capacity_ah: p.capacity_ah,
        warranty_months: p.warranty_months,
        selling_price: p.selling_price,
        in_stock: p.quantity_in_stock > 0,
        unit: p.unit,
        image_url: p.image_url,
        images: product_images,
        suggested_products: None,
    }
}

pub async fn fetch_suggested_products(
    state: &AppState,
    exclude_id: Uuid,
) -> Result<Vec<PublicProduct>, StatusCode> {
    let rows = sqlx::query_as!(
        PublicProductRow,
        r#"SELECT id, name, slug, sku, brand_id, model, description, image_url as "image_url!",
              power_rating_watts, voltage_rating, capacity_ah, warranty_months,
              selling_price, quantity_in_stock, unit
       FROM products
       WHERE is_active = TRUE AND id != $1
       ORDER BY random()
       LIMIT $2"#,
        exclude_id,
        SUGGESTED_PRODUCTS_LIMIT
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let product_ids: Vec<Uuid> = rows.iter().map(|p| p.id).collect();

    let images = sqlx::query_as!(
        Image,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, created_at
           FROM images
           WHERE product_id = ANY($1)
           ORDER BY created_at ASC"#,
        &product_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let brand_ids: Vec<Uuid> = rows.iter().filter_map(|p| p.brand_id).collect();
    let brand_map = fetch_product_brands(state, &brand_ids).await?;

    Ok(rows
        .into_iter()
        .map(|p| build_public_product(p, &images, &brand_map))
        .collect())
}

pub async fn get_public_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<PublicProduct>>, StatusCode> {
    let products = sqlx::query_as!(
        PublicProductRow,
        r#"SELECT id, name, slug, sku, brand_id, model, description, image_url as "image_url!",
              power_rating_watts, voltage_rating, capacity_ah, warranty_months,
              selling_price, quantity_in_stock, unit
       FROM products
       WHERE is_active = TRUE
       ORDER BY created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let product_ids: Vec<Uuid> = products.iter().map(|p| p.id).collect();

    let images = sqlx::query_as!(
        Image,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, created_at
           FROM images
           WHERE product_id = ANY($1)
           ORDER BY created_at ASC"#,
        &product_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let brand_ids: Vec<Uuid> = products.iter().filter_map(|p| p.brand_id).collect();
    let brand_map = fetch_product_brands(&state, &brand_ids).await?;

    let result = products
        .into_iter()
        .map(|p| build_public_product(p, &images, &brand_map))
        .collect();

    Ok(Json(result))
}

pub async fn get_public_product(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<PublicProduct>, StatusCode> {
    let p = sqlx::query_as!(
        PublicProductRow,
        r#"SELECT id, name, slug, sku, brand_id, model, description, image_url as "image_url!",
              power_rating_watts, voltage_rating, capacity_ah, warranty_months,
              selling_price, quantity_in_stock, unit
       FROM products
       WHERE slug = $1 AND is_active = TRUE"#,
        slug
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let images = sqlx::query_as!(
        Image,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, created_at
           FROM images
           WHERE product_id = $1
           ORDER BY created_at ASC"#,
        p.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let brand_ids: Vec<Uuid> = p.brand_id.into_iter().collect();
    let brand_map = fetch_product_brands(&state, &brand_ids).await?;

    let product_id = p.id;
    let mut product = build_public_product(p, &images, &brand_map);
    product.suggested_products = Some(fetch_suggested_products(&state, product_id).await?);

    Ok(Json(product))
}
