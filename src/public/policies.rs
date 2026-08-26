use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use crate::AppState;
use crate::dashboard::policies::Policy;

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
