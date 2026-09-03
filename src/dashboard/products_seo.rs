use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::AppState;
use crate::auth::Claims;
use crate::utils::*;

#[derive(Debug, Clone, Serialize)]
pub struct ProductInfo {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct ProductSeo {
    pub id: Uuid,
    pub product_id: Uuid,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image_url: Option<String>,
    pub canonical_url: Option<String>,
    pub focus_keyword: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ProductSeoWithProduct {
    pub id: Uuid,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image_url: Option<String>,
    pub canonical_url: Option<String>,
    pub focus_keyword: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub product: ProductInfo,
}

struct ProductSeoRow {
    id: Uuid,
    product_id: Uuid,
    meta_title: Option<String>,
    meta_description: Option<String>,
    meta_keywords: Option<String>,
    og_title: Option<String>,
    og_description: Option<String>,
    og_image_url: Option<String>,
    canonical_url: Option<String>,
    focus_keyword: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    product_name: String,
    product_slug: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateProductSeo {
    pub product_id: Uuid,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image_url: Option<String>,
    pub canonical_url: Option<String>,
    pub focus_keyword: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProductSeo {
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image_url: Option<String>,
    pub canonical_url: Option<String>,
    pub focus_keyword: Option<String>,
}

fn to_response(r: ProductSeoRow) -> ProductSeoWithProduct {
    ProductSeoWithProduct {
        id: r.id,
        meta_title: r.meta_title,
        meta_description: r.meta_description,
        meta_keywords: r.meta_keywords,
        og_title: r.og_title,
        og_description: r.og_description,
        og_image_url: r.og_image_url,
        canonical_url: r.canonical_url,
        focus_keyword: r.focus_keyword,
        created_at: r.created_at,
        updated_at: r.updated_at,
        product: ProductInfo {
            id: r.product_id,
            name: r.product_name,
            slug: r.product_slug,
        },
    }
}

pub async fn get_products_seo(
    State(state): State<AppState>,
) -> Result<Json<Vec<ProductSeoWithProduct>>, StatusCode> {
    let rows = sqlx::query_as!(
        ProductSeoRow,
        r#"SELECT ps.id, ps.product_id, ps.meta_title, ps.meta_description, ps.meta_keywords,
                  ps.og_title, ps.og_description, ps.og_image_url, ps.canonical_url, ps.focus_keyword,
                  ps.created_at, ps.updated_at,
                  p.name as product_name, p.slug as product_slug
           FROM products_seo ps
           JOIN products p ON p.id = ps.product_id
           ORDER BY ps.created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = rows.into_iter().map(to_response).collect();

    Ok(Json(result))
}

pub async fn get_product_seo(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<ProductSeoWithProduct>, StatusCode> {
    let row = sqlx::query_as!(
        ProductSeoRow,
        r#"SELECT ps.id, ps.product_id, ps.meta_title, ps.meta_description, ps.meta_keywords,
                  ps.og_title, ps.og_description, ps.og_image_url, ps.canonical_url, ps.focus_keyword,
                  ps.created_at, ps.updated_at,
                  p.name as product_name, p.slug as product_slug
           FROM products_seo ps
           JOIN products p ON p.id = ps.product_id
           WHERE ps.id = $1"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(to_response(row)))
}

pub async fn create_product_seo(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateProductSeo>,
) -> Result<(StatusCode, Json<ProductSeoWithProduct>), StatusCode> {
    let row = sqlx::query_as!(
        ProductSeoRow,
        r#"INSERT INTO products_seo (
               product_id, meta_title, meta_description, meta_keywords,
               og_title, og_description, og_image_url, canonical_url, focus_keyword
           )
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
           RETURNING
               products_seo.id, products_seo.product_id, products_seo.meta_title,
               products_seo.meta_description, products_seo.meta_keywords, products_seo.og_title,
               products_seo.og_description, products_seo.og_image_url, products_seo.canonical_url,
               products_seo.focus_keyword, products_seo.created_at, products_seo.updated_at,
               (SELECT name FROM products WHERE id = products_seo.product_id) as "product_name!",
               (SELECT slug FROM products WHERE id = products_seo.product_id) as "product_slug!""#,
        payload.product_id,
        payload.meta_title,
        payload.meta_description,
        payload.meta_keywords,
        payload.og_title,
        payload.og_description,
        payload.og_image_url,
        payload.canonical_url,
        payload.focus_keyword
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

    log_audit(
        &state.db,
        Some(claims.sub),
        "create",
        "product_seo",
        Some(row.id),
        "created",
        Some(json!({ "product_id": row.product_id, "meta_title": row.meta_title })),
    )
    .await;

    Ok((StatusCode::CREATED, Json(to_response(row))))
}

pub async fn update_product_seo(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateProductSeo>,
) -> Result<Json<ProductSeoWithProduct>, StatusCode> {
    let row = sqlx::query_as!(
        ProductSeoRow,
        r#"UPDATE products_seo ps
           SET meta_title = COALESCE($1, meta_title),
               meta_description = COALESCE($2, meta_description),
               meta_keywords = COALESCE($3, meta_keywords),
               og_title = COALESCE($4, og_title),
               og_description = COALESCE($5, og_description),
               og_image_url = COALESCE($6, og_image_url),
               canonical_url = COALESCE($7, canonical_url),
               focus_keyword = COALESCE($8, focus_keyword),
               updated_at = NOW()
           WHERE ps.id = $9
           RETURNING
               ps.id, ps.product_id, ps.meta_title, ps.meta_description, ps.meta_keywords,
               ps.og_title, ps.og_description, ps.og_image_url, ps.canonical_url, ps.focus_keyword,
               ps.created_at, ps.updated_at,
               (SELECT name FROM products WHERE id = ps.product_id) as "product_name!",
               (SELECT slug FROM products WHERE id = ps.product_id) as "product_slug!""#,
        payload.meta_title,
        payload.meta_description,
        payload.meta_keywords,
        payload.og_title,
        payload.og_description,
        payload.og_image_url,
        payload.canonical_url,
        payload.focus_keyword,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    log_audit(
        &state.db,
        Some(claims.sub),
        "update",
        "product_seo",
        Some(row.id),
        "updated",
        Some(json!({ "product_id": row.product_id, "meta_title": row.meta_title })),
    )
    .await;

    Ok(Json(to_response(row)))
}

pub async fn delete_product_seo(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM products_seo WHERE id = $1", uuid)
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
        "product_seo",
        Some(uuid),
        "deleted",
        None,
    )
    .await;

    Ok(StatusCode::NO_CONTENT)
}
