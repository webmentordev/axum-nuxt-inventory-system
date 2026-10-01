use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ProductSpecification {
    pub id: Uuid,
    pub product_id: Uuid,
    pub group_name: Option<String>,
    pub key: String,
    pub value: String,
    pub unit: Option<String>,
    pub sort_order: i32,
    pub is_highlighted: bool,
    pub is_filterable: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSpecification {
    pub group_name: Option<String>,
    pub key: String,
    pub value: String,
    pub unit: Option<String>,
    pub sort_order: Option<i32>,
    pub is_highlighted: Option<bool>,
    pub is_filterable: Option<bool>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSpecifications {
    pub product_id: Uuid,
    pub specifications: Vec<CreateSpecification>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSpecification {
    pub group_name: Option<String>,
    pub key: Option<String>,
    pub value: Option<String>,
    pub unit: Option<String>,
    pub sort_order: Option<i32>,
    pub is_highlighted: Option<bool>,
    pub is_filterable: Option<bool>,
    pub is_active: Option<bool>,
}

fn map_db_error(e: sqlx::Error) -> StatusCode {
    if let sqlx::Error::Database(db) = &e {
        match db.code().as_deref() {
            Some("23505") => return StatusCode::CONFLICT,
            Some("23503") => return StatusCode::NOT_FOUND,
            _ => {}
        }
    }
    StatusCode::INTERNAL_SERVER_ERROR
}

pub async fn get_specifications(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductSpecification>>, StatusCode> {
    let specs = sqlx::query_as!(
        ProductSpecification,
        r#"SELECT id, product_id, group_name, key, value, unit, sort_order,
                  is_highlighted, is_filterable, is_active, created_at, updated_at
           FROM product_specifications
           ORDER BY created_at DESC, sort_order ASC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(map_db_error)?;

    Ok(Json(specs))
}

pub async fn create_specifications(
    State(state): State<AppState>,
    Json(body): Json<CreateSpecifications>,
) -> Result<(StatusCode, Json<Vec<ProductSpecification>>), StatusCode> {
    let product_id = body.product_id;
    let items = body.specifications;

    if items.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if items
        .iter()
        .any(|i| i.key.trim().is_empty() || i.value.trim().is_empty())
    {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut tx = state.db.begin().await.map_err(map_db_error)?;
    let mut created = Vec::with_capacity(items.len());

    for (index, item) in items.into_iter().enumerate() {
        let spec = sqlx::query_as!(
            ProductSpecification,
            r#"INSERT INTO product_specifications
                   (product_id, group_name, key, value, unit, sort_order,
                    is_highlighted, is_filterable, is_active)
               VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
               RETURNING id, product_id, group_name, key, value, unit, sort_order,
                         is_highlighted, is_filterable, is_active, created_at, updated_at"#,
            product_id,
            item.group_name.as_deref().map(str::trim),
            item.key.trim(),
            item.value.trim(),
            item.unit.as_deref().map(str::trim),
            item.sort_order.unwrap_or(index as i32),
            item.is_highlighted.unwrap_or(false),
            item.is_filterable.unwrap_or(false),
            item.is_active.unwrap_or(true)
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(map_db_error)?;

        created.push(spec);
    }

    tx.commit().await.map_err(map_db_error)?;

    Ok((StatusCode::CREATED, Json(created)))
}

pub async fn update_specification(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateSpecification>,
) -> Result<Json<ProductSpecification>, StatusCode> {
    let key = body.key.as_deref().map(str::trim);
    let value = body.value.as_deref().map(str::trim);

    if key.is_some_and(str::is_empty) || value.is_some_and(str::is_empty) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let spec = sqlx::query_as!(
        ProductSpecification,
        r#"UPDATE product_specifications
           SET group_name = COALESCE($2, group_name),
               key = COALESCE($3, key),
               value = COALESCE($4, value),
               unit = COALESCE($5, unit),
               sort_order = COALESCE($6, sort_order),
               is_highlighted = COALESCE($7, is_highlighted),
               is_filterable = COALESCE($8, is_filterable),
               is_active = COALESCE($9, is_active),
               updated_at = NOW()
           WHERE id = $1
           RETURNING id, product_id, group_name, key, value, unit, sort_order,
                     is_highlighted, is_filterable, is_active, created_at, updated_at"#,
        id,
        body.group_name.as_deref().map(str::trim),
        key,
        value,
        body.unit.as_deref().map(str::trim),
        body.sort_order,
        body.is_highlighted,
        body.is_filterable,
        body.is_active
    )
    .fetch_optional(&state.db)
    .await
    .map_err(map_db_error)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(spec))
}
