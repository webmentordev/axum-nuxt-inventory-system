use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{AppState, public::products::*};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicSubCategoryListItem {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct PublicSubCategoryDetail {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub products: Vec<PublicProductCard>,
    pub has_more: bool,
}

struct SubCategoryDetailRow {
    id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
}

pub async fn get_public_sub_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<PublicSubCategoryListItem>>, StatusCode> {
    let sub_categories = sqlx::query_as!(
        PublicSubCategoryListItem,
        r#"SELECT name, slug
           FROM sub_categories
           WHERE is_active = TRUE
           ORDER BY name ASC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(sub_categories))
}

pub async fn get_public_sub_category(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(params): Query<PublicProductsQuery>,
) -> Result<Json<PublicSubCategoryDetail>, StatusCode> {
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = params.offset.unwrap_or(0).max(0);

    let sub_category = sqlx::query_as!(
        SubCategoryDetailRow,
        r#"SELECT id, name, slug, description
           FROM sub_categories
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
        WHERE sub_category_id = $1 AND is_active = TRUE
        ORDER BY created_at DESC, id DESC
        LIMIT $2 OFFSET $3"#,
        sub_category.id,
        limit + 1,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let has_more = rows.len() as i64 > limit;
    rows.truncate(limit as usize);

    let products = cards_from_rows(&state, rows).await?;

    Ok(Json(PublicSubCategoryDetail {
        name: sub_category.name,
        slug: sub_category.slug,
        description: sub_category.description,
        products,
        has_more,
    }))
}
