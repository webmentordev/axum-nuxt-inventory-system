use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::utils::slugify;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SubCategory {
    pub id: Uuid,
    pub category_id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSubCategory {
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSubCategory {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

pub async fn get_sub_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<SubCategory>>, StatusCode> {
    let sub_categories = sqlx::query_as!(
        SubCategory,
        r#"SELECT id, category_id, name, slug, description, is_active, created_at, updated_at
           FROM sub_categories
           ORDER BY created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(sub_categories))
}

pub async fn create_sub_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateSubCategory>,
) -> Result<(StatusCode, Json<SubCategory>), StatusCode> {
    let slug = slugify(&payload.name);

    let sub_category = sqlx::query_as!(
        SubCategory,
        r#"INSERT INTO sub_categories (category_id, name, slug, description)
           VALUES ($1, $2, $3, $4)
           RETURNING id, category_id, name, slug, description, is_active, created_at, updated_at"#,
        payload.category_id,
        payload.name,
        slug,
        payload.description
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

    Ok((StatusCode::CREATED, Json(sub_category)))
}

pub async fn get_sub_category(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<SubCategory>, StatusCode> {
    let sub_category = sqlx::query_as!(
        SubCategory,
        r#"SELECT id, category_id, name, slug, description, is_active, created_at, updated_at
           FROM sub_categories
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(sub_category))
}

pub async fn update_sub_category(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateSubCategory>,
) -> Result<Json<SubCategory>, StatusCode> {
    let new_slug = payload.name.as_ref().map(|name| slugify(name));

    let sub_category = sqlx::query_as!(
        SubCategory,
        r#"UPDATE sub_categories
           SET name = COALESCE($1, name),
               slug = COALESCE($2, slug),
               description = COALESCE($3, description),
               is_active = COALESCE($4, is_active),
               updated_at = NOW()
           WHERE id = $5
           RETURNING id, category_id, name, slug, description, is_active, created_at, updated_at"#,
        payload.name,
        new_slug,
        payload.description,
        payload.is_active,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(sub_category))
}

pub async fn delete_sub_category(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM sub_categories WHERE id = $1", uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
