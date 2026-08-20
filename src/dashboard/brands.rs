use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dashboard::images::{Image, WithFullUrl};
use crate::{AppState, utils::slugify};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Brand {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBrand {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateBrand {
    pub name: Option<String>,
    pub description: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BrandWithDetails {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub products_count: i64,
    pub images: Vec<Image>,
}

struct BrandRow {
    id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    products_count: i64,
}

pub async fn get_brands(
    State(state): State<AppState>,
) -> Result<Json<Vec<BrandWithDetails>>, StatusCode> {
    let rows = sqlx::query_as!(
        BrandRow,
        r#"SELECT b.id, b.name, b.slug, b.description, b.is_active, b.created_at, b.updated_at,
                  COUNT(p.id) as "products_count!"
           FROM brands b
           LEFT JOIN products p ON p.brand_id = b.id
           GROUP BY b.id
           ORDER BY b.created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();

    let images = sqlx::query_as!(
        Image,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, created_at
           FROM images
           WHERE brand_id = ANY($1)"#,
        &ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let brands = rows
        .into_iter()
        .map(|r| {
            let brand_images = images
                .iter()
                .filter(|img| img.brand_id == Some(r.id))
                .cloned()
                .map(Image::with_full_url)
                .collect();

            BrandWithDetails {
                id: r.id,
                name: r.name,
                slug: r.slug,
                description: r.description,
                is_active: r.is_active,
                created_at: r.created_at,
                updated_at: r.updated_at,
                products_count: r.products_count,
                images: brand_images,
            }
        })
        .collect();

    Ok(Json(brands))
}

pub async fn get_brand(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<BrandWithDetails>, StatusCode> {
    let row = sqlx::query_as!(
        BrandRow,
        r#"SELECT b.id, b.name, b.slug, b.description, b.is_active, b.created_at, b.updated_at,
                  COUNT(p.id) as "products_count!"
           FROM brands b
           LEFT JOIN products p ON p.brand_id = b.id
           WHERE b.id = $1
           GROUP BY b.id"#,
        uuid
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let images = sqlx::query_as!(
        Image,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, created_at
           FROM images
           WHERE brand_id = $1"#,
        uuid
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let images = images.into_iter().map(Image::with_full_url).collect();

    Ok(Json(BrandWithDetails {
        id: row.id,
        name: row.name,
        slug: row.slug,
        description: row.description,
        is_active: row.is_active,
        created_at: row.created_at,
        updated_at: row.updated_at,
        products_count: row.products_count,
        images,
    }))
}

pub async fn create_brand(
    State(state): State<AppState>,
    Json(payload): Json<CreateBrand>,
) -> Result<(StatusCode, Json<Brand>), StatusCode> {
    let slug = slugify(&payload.name);

    let brand = sqlx::query_as!(
        Brand,
        r#"INSERT INTO brands (name, slug, description)
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

    Ok((StatusCode::CREATED, Json(brand)))
}

pub async fn update_brand(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateBrand>,
) -> Result<Json<Brand>, StatusCode> {
    let new_slug = payload.name.as_ref().map(|name| slugify(name));

    let brand = sqlx::query_as!(
        Brand,
        r#"UPDATE brands
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

    Ok(Json(brand))
}

pub async fn delete_brand(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM brands WHERE id = $1", uuid)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
