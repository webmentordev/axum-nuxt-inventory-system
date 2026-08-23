use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;

use crate::AppState;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicBrandListItem {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicBrandDetail {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
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
) -> Result<Json<PublicBrandDetail>, StatusCode> {
    let brand = sqlx::query_as!(
        PublicBrandDetail,
        r#"SELECT name, slug, description
           FROM brands
           WHERE slug = $1 AND is_active = TRUE"#,
        slug
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(brand))
}
