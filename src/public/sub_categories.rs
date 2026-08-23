use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;

use crate::AppState;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicSubCategoryListItem {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicSubCategoryDetail {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
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
) -> Result<Json<PublicSubCategoryDetail>, StatusCode> {
    let sub_category = sqlx::query_as!(
        PublicSubCategoryDetail,
        r#"SELECT name, slug, description
           FROM sub_categories
           WHERE slug = $1 AND is_active = TRUE"#,
        slug
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(sub_category))
}
