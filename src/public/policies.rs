use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;

use crate::AppState;
use crate::dashboard::policies::Policy;

#[derive(Serialize)]
pub struct PolicyListItem {
    pub name: String,
    pub slug: String,
    pub sort_order: i32,
}

pub async fn get_public_policy(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Policy>, StatusCode> {
    let policy = sqlx::query_as!(
        Policy,
        r#"SELECT id, name, slug, seo_title, seo_description, content,
                  is_active, sort_order, created_at, updated_at
           FROM policies
           WHERE slug = $1 AND is_active = TRUE"#,
        slug
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(policy))
}

pub async fn get_public_policies(
    State(state): State<AppState>,
) -> Result<Json<Vec<PolicyListItem>>, StatusCode> {
    let policies = sqlx::query_as!(
        PolicyListItem,
        r#"SELECT name, slug, sort_order
           FROM policies
           WHERE is_active = TRUE
           ORDER BY sort_order ASC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(policies))
}
