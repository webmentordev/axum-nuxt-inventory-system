use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;
use crate::dashboard::categories::Category;
use crate::dashboard::images::{Image, WithFullUrl};
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

#[derive(Debug, Clone, Serialize)]
pub struct SubCategoryWithDetails {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub products_count: i64,
    pub category: Category,
    pub images: Vec<Image>,
}

struct SubCategoryRow {
    id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
    is_active: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    products_count: i64,
    category_id: Uuid,
    category_name: String,
    category_slug: String,
    category_description: Option<String>,
    category_is_active: bool,
    category_created_at: DateTime<Utc>,
    category_updated_at: DateTime<Utc>,
}

impl From<SubCategoryRow> for SubCategoryWithDetails {
    fn from(r: SubCategoryRow) -> Self {
        Self {
            id: r.id,
            name: r.name,
            slug: r.slug,
            description: r.description,
            is_active: r.is_active,
            created_at: r.created_at,
            updated_at: r.updated_at,
            products_count: r.products_count,
            category: Category {
                id: r.category_id,
                name: r.category_name,
                slug: r.category_slug,
                description: r.category_description,
                is_active: r.category_is_active,
                created_at: r.category_created_at,
                updated_at: r.category_updated_at,
            },
            images: Vec::new(),
        }
    }
}

pub async fn get_sub_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<SubCategoryWithDetails>>, StatusCode> {
    let rows = sqlx::query_as!(
        SubCategoryRow,
        r#"SELECT sc.id, sc.name, sc.slug, sc.description, sc.is_active, sc.created_at, sc.updated_at,
                  COUNT(DISTINCT p.id) as "products_count!",
                  c.id as "category_id!", c.name as "category_name!", c.slug as "category_slug!",
                  c.description as category_description, c.is_active as "category_is_active!",
                  c.created_at as "category_created_at!", c.updated_at as "category_updated_at!"
           FROM sub_categories sc
           JOIN categories c ON c.id = sc.category_id
           LEFT JOIN products p ON p.sub_category_id = sc.id
           GROUP BY sc.id, c.id
           ORDER BY sc.created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();

    let images = sqlx::query_as!(
        Image,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, created_at
           FROM images
           WHERE sub_category_id = ANY($1)"#,
        &ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let sub_categories = rows
        .into_iter()
        .map(|r| {
            let sc_images = images
                .iter()
                .filter(|img| img.sub_category_id == Some(r.id))
                .cloned()
                .map(Image::with_full_url)
                .collect();

            let mut details = SubCategoryWithDetails::from(r);
            details.images = sc_images;
            details
        })
        .collect();

    Ok(Json(sub_categories))
}

pub async fn get_sub_category(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<SubCategoryWithDetails>, StatusCode> {
    let row = sqlx::query_as!(
        SubCategoryRow,
        r#"SELECT sc.id, sc.name, sc.slug, sc.description, sc.is_active, sc.created_at, sc.updated_at,
                  COUNT(DISTINCT p.id) as "products_count!",
                  c.id as "category_id!", c.name as "category_name!", c.slug as "category_slug!",
                  c.description as category_description, c.is_active as "category_is_active!",
                  c.created_at as "category_created_at!", c.updated_at as "category_updated_at!"
           FROM sub_categories sc
           JOIN categories c ON c.id = sc.category_id
           LEFT JOIN products p ON p.sub_category_id = sc.id
           WHERE sc.id = $1
           GROUP BY sc.id, c.id"#,
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
           WHERE sub_category_id = $1"#,
        uuid
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut details = SubCategoryWithDetails::from(row);
    let images = images.into_iter().map(Image::with_full_url).collect();
    details.images = images;

    Ok(Json(details))
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
