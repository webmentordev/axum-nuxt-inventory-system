use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, utils::slugify};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Policy {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub content: Option<String>,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePolicy {
    pub name: Option<String>,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub content: Option<String>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePolicy {
    pub name: String,
    pub seo_title: String,
    pub seo_description: String,
    pub content: String,
    pub is_active: bool,
    pub sort_order: i32,
}

pub async fn create_policy(
    State(state): State<AppState>,
    Json(payload): Json<CreatePolicy>,
) -> Result<(StatusCode, Json<Policy>), StatusCode> {
    let slug = slugify(&payload.name, false);
    let is_active = payload.is_active;
    let sort_order = payload.sort_order;

    let policy = sqlx::query_as!(
        Policy,
        r#"INSERT INTO policies (name, slug, seo_title, seo_description, content, is_active, sort_order)
           VALUES ($1, $2, $3, $4, $5, $6, $7)
           RETURNING id, name, slug, seo_title, seo_description, content,
                     is_active, sort_order, created_at, updated_at"#,
        payload.name,
        slug,
        payload.seo_title,
        payload.seo_description,
        payload.content,
        is_active,
        sort_order
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    Ok((StatusCode::CREATED, Json(policy)))
}

pub async fn get_policies(State(state): State<AppState>) -> Result<Json<Vec<Policy>>, StatusCode> {
    let policies = sqlx::query_as!(
        Policy,
        r#"SELECT id, name, slug, seo_title, seo_description, content,
                  is_active, sort_order, created_at, updated_at
           FROM policies
           ORDER BY sort_order ASC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(policies))
}

pub async fn update_policy(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdatePolicy>,
) -> Result<Json<Policy>, StatusCode> {
    let new_slug = payload.name.as_ref().map(|name| slugify(name, false));

    let policy = sqlx::query_as!(
        Policy,
        r#"UPDATE policies
           SET name = COALESCE($1, name),
               slug = COALESCE($2, slug),
               seo_title = COALESCE($3, seo_title),
               seo_description = COALESCE($4, seo_description),
               content = COALESCE($5, content),
               is_active = COALESCE($6, is_active),
               sort_order = COALESCE($7, sort_order),
               updated_at = NOW()
           WHERE id = $8
           RETURNING id, name, slug, seo_title, seo_description, content,
                     is_active, sort_order, created_at, updated_at"#,
        payload.name,
        new_slug,
        payload.seo_title,
        payload.seo_description,
        payload.content,
        payload.is_active,
        payload.sort_order,
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

    Ok(Json(policy))
}

pub async fn delete_policy(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM policies WHERE id = $1", uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
