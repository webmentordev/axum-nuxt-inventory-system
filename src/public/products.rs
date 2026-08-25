use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

use crate::{
    AppState,
    dashboard::images::{Image, WithFullUrl},
};

#[derive(Debug, Serialize, sqlx::FromRow)]
struct PublicProductRow {
    id: Uuid,
    name: String,
    slug: String,
    sku: String,
    brand: Option<String>,
    model: Option<String>,
    description: Option<String>,
    power_rating_watts: Option<Decimal>,
    voltage_rating: Option<Decimal>,
    capacity_ah: Option<Decimal>,
    warranty_months: Option<i16>,
    selling_price: Decimal,
    quantity_in_stock: i32,
    image_url: String,
    unit: String,
}

#[derive(Debug, Serialize)]
pub struct PublicProduct {
    pub id: Uuid,
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
    pub selling_price: Decimal,
    pub in_stock: bool,
    pub unit: String,
    pub image_url: String,
    pub images: Vec<Image>,
}

pub async fn get_public_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<PublicProduct>>, StatusCode> {
    let products = sqlx::query_as!(
        PublicProductRow,
        r#"SELECT id, name, slug, sku, brand, model, description, image_url as "image_url!",
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

    let result = products
        .into_iter()
        .map(|p| {
            let product_images = images
                .iter()
                .filter(|img| img.product_id == Some(p.id))
                .cloned()
                .map(Image::with_full_url)
                .collect();

            PublicProduct {
                id: p.id,
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
                selling_price: p.selling_price,
                in_stock: p.quantity_in_stock > 0,
                unit: p.unit,
                image_url: p.image_url,
                images: product_images,
            }
        })
        .collect();

    Ok(Json(result))
}

pub async fn get_public_product(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<PublicProduct>, StatusCode> {
    let p = sqlx::query_as!(
        PublicProductRow,
        r#"SELECT id, name, slug, sku, brand, model, description, image_url as "image_url!",
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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .into_iter()
    .map(Image::with_full_url)
    .collect();

    Ok(Json(PublicProduct {
        id: p.id,
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
        selling_price: p.selling_price,
        in_stock: p.quantity_in_stock > 0,
        unit: p.unit,
        image_url: p.image_url,
        images,
    }))
}
