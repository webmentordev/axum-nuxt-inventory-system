use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{AppState, public::products::*};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicBrandListItem {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct PublicBrandDetail {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub products: Vec<PublicProductCard>,
    pub has_more: bool,
}

struct BrandDetailRow {
    id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
}

pub async fn get_public_brands(
    State(state): State<AppState>,
) -> Result<Json<Vec<PublicBrandListItem>>, StatusCode> {
    let brands = sqlx::query_as!(
        PublicBrandListItem,
        r#"SELECT name, slug
           FROM brands
           WHERE is_active = TRUE
           ORDER BY name ASC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(brands))
}

pub async fn get_public_brand(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(params): Query<PublicProductsQuery>,
) -> Result<Json<PublicBrandDetail>, StatusCode> {
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = params.offset.unwrap_or(0).max(0);

    let brand = sqlx::query_as!(
        BrandDetailRow,
        r#"SELECT id, name, slug, description
           FROM brands
           WHERE slug = $1 AND is_active = TRUE"#,
        slug
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let mut rows = sqlx::query_as!(
        PublicProductCardRow,
        r#"SELECT name, slug, brand_id, model,
                selling_price, compare_at_selling_price, quantity_in_stock,
                COALESCE(image_url, '') as "image_url!"
        FROM products
        WHERE brand_id = $1 AND is_active = TRUE
        ORDER BY created_at DESC, id DESC
        LIMIT $2 OFFSET $3"#,
        brand.id,
        limit + 1,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let has_more = rows.len() as i64 > limit;
    rows.truncate(limit as usize);

    let products = cards_from_rows(&state, rows).await?;

    Ok(Json(PublicBrandDetail {
        name: brand.name,
        slug: brand.slug,
        description: brand.description,
        products,
        has_more,
    }))
}
