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
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategory {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

pub async fn get_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<Category>>, StatusCode> {
    let categories = sqlx::query_as!(
        Category,
        r#"SELECT id, name, slug, description, is_active, created_at, updated_at
           FROM categories
           ORDER BY created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(categories))
}

pub async fn get_category(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<Category>, StatusCode> {
    let category = sqlx::query_as!(
        Category,
        r#"SELECT id, name, slug, description, is_active, created_at, updated_at
           FROM categories
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(category))
}

pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategory>,
) -> Result<(StatusCode, Json<Category>), StatusCode> {
    let slug = slugify(&payload.name);

    let category = sqlx::query_as!(
        Category,
        r#"INSERT INTO categories (name, slug, description)
           VALUES ($1, $2, $3)
           RETURNING id, name, slug, description, is_active, created_at, updated_at"#,
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
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::CREATED, Json(category)))
}

pub async fn update_category(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateCategory>,
) -> Result<Json<Category>, StatusCode> {
    let new_slug = payload.name.as_ref().map(|name| slugify(name));

    let category = sqlx::query_as!(
        Category,
        r#"UPDATE categories
           SET name = COALESCE($1, name),
               slug = COALESCE($2, slug),
               description = COALESCE($3, description),
               is_active = COALESCE($4, is_active),
               updated_at = NOW()
           WHERE id = $5
           RETURNING id, name, slug, description, is_active, created_at, updated_at"#,
        payload.name,
        new_slug,
        payload.description,
        payload.is_active,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(category))
}

pub async fn delete_category(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM categories WHERE id = $1", uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
