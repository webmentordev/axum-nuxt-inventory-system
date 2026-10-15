use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::auth::Claims;
use crate::{AppState, utils::*};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct InfoCard {
    pub id: Uuid,
    pub offer_title: String,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
    pub path_url: String,
    pub is_active: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateInfoCard {
    pub offer_title: String,
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
    pub path_url: String,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateInfoCard {
    pub offer_title: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub path_url: Option<String>,
    pub is_active: Option<bool>,
    pub sort_order: Option<i32>,
}

fn map_db_error(err: sqlx::Error) -> StatusCode {
    match &err {
        sqlx::Error::Database(db_err) => match db_err.code().as_deref() {
            Some("23505") => StatusCode::CONFLICT,
            Some("23514") | Some("22001") => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        },
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

pub async fn get_info_cards(
    State(state): State<AppState>,
) -> Result<Json<Vec<InfoCard>>, StatusCode> {
    let cards = sqlx::query_as!(
        InfoCard,
        r#"SELECT id, offer_title, title, description, image_url, path_url,
                  is_active, sort_order, created_at, updated_at
           FROM info_cards
           ORDER BY sort_order ASC, created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(cards))
}

pub async fn get_info_card(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<InfoCard>, StatusCode> {
    let card = sqlx::query_as!(
        InfoCard,
        r#"SELECT id, offer_title, title, description, image_url, path_url,
                  is_active, sort_order, created_at, updated_at
           FROM info_cards
           WHERE id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(card))
}

pub async fn create_info_card(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateInfoCard>,
) -> Result<(StatusCode, Json<InfoCard>), StatusCode> {
    let card = sqlx::query_as!(
        InfoCard,
        r#"INSERT INTO info_cards (offer_title, title, description, image_url, path_url, is_active, sort_order)
           VALUES ($1, $2, $3, $4, $5, COALESCE($6, TRUE), COALESCE($7, 0))
           RETURNING id, offer_title, title, description, image_url, path_url,
                     is_active, sort_order, created_at, updated_at"#,
        payload.offer_title,
        payload.title,
        payload.description,
        payload.image_url,
        payload.path_url,
        payload.is_active,
        payload.sort_order
    )
    .fetch_one(&state.db)
    .await
    .map_err(map_db_error)?;

    log_audit(
        &state.db,
        Some(claims.sub),
        "create",
        "info_card",
        Some(card.id),
        "created",
        Some(json!({
            "offer_title": card.offer_title,
            "title": card.title,
            "sort_order": card.sort_order
        })),
    )
    .await;

    Ok((StatusCode::CREATED, Json(card)))
}

pub async fn update_info_card(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateInfoCard>,
) -> Result<Json<InfoCard>, StatusCode> {
    let card = sqlx::query_as!(
        InfoCard,
        r#"UPDATE info_cards
           SET offer_title = COALESCE($1, offer_title),
               title = COALESCE($2, title),
               description = COALESCE($3, description),
               image_url = COALESCE($4, image_url),
               path_url = COALESCE($5, path_url),
               is_active = COALESCE($6, is_active),
               sort_order = COALESCE($7, sort_order),
               updated_at = NOW()
           WHERE id = $8
           RETURNING id, offer_title, title, description, image_url, path_url,
                     is_active, sort_order, created_at, updated_at"#,
        payload.offer_title,
        payload.title,
        payload.description,
        payload.image_url,
        payload.path_url,
        payload.is_active,
        payload.sort_order,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(map_db_error)?
    .ok_or(StatusCode::NOT_FOUND)?;

    log_audit(
        &state.db,
        Some(claims.sub),
        "update",
        "info_card",
        Some(card.id),
        "updated",
        Some(json!({
            "offer_title": card.offer_title,
            "title": card.title,
            "is_active": card.is_active,
            "sort_order": card.sort_order
        })),
    )
    .await;

    Ok(Json(card))
}

pub async fn delete_info_card(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM info_cards WHERE id = $1", uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    log_audit(
        &state.db,
        Some(claims.sub),
        "delete",
        "info_card",
        Some(uuid),
        "deleted",
        None,
    )
    .await;

    Ok(StatusCode::NO_CONTENT)
}
