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
use crate::dashboard::categories::Category;
use crate::dashboard::uploads::{Upload, WithFullUrl};
use crate::utils::*;

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
    pub category_id: Option<Uuid>,
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
    pub uploads: Vec<Upload>,
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
    category_is_featured: bool,
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
                is_featured: r.category_is_featured,
                created_at: r.category_created_at,
                updated_at: r.category_updated_at,
            },
            uploads: Vec::new(),
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
                c.is_featured as "category_is_featured!",
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

    let uploads = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           WHERE sub_category_id = ANY($1)"#,
        &ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let sub_categories = rows
        .into_iter()
        .map(|r| {
            let sc_uploads = uploads
                .iter()
                .filter(|u| u.sub_category_id == Some(r.id))
                .cloned()
                .map(Upload::with_full_url)
                .collect();

            let mut details = SubCategoryWithDetails::from(r);
            details.uploads = sc_uploads;
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
                c.is_featured as "category_is_featured!",
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

    let uploads = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           WHERE sub_category_id = $1"#,
        uuid
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut details = SubCategoryWithDetails::from(row);
    let uploads = uploads.into_iter().map(Upload::with_full_url).collect();
    details.uploads = uploads;

    Ok(Json(details))
}

pub async fn get_sub_categories_by_category(
    State(state): State<AppState>,
    Path(category_id): Path<Uuid>,
) -> Result<Json<Vec<SubCategoryWithDetails>>, StatusCode> {
    let rows = sqlx::query_as!(
        SubCategoryRow,
        r#"SELECT sc.id, sc.name, sc.slug, sc.description, sc.is_active, sc.created_at, sc.updated_at,
                  COUNT(DISTINCT p.id) as "products_count!",
                  c.id as "category_id!", c.name as "category_name!", c.slug as "category_slug!",
                  c.description as category_description, c.is_active as "category_is_active!",
                  c.is_featured as "category_is_featured!",
                  c.created_at as "category_created_at!", c.updated_at as "category_updated_at!"
           FROM sub_categories sc
           JOIN categories c ON c.id = sc.category_id
           LEFT JOIN products p ON p.sub_category_id = sc.id
           WHERE sc.category_id = $1
           GROUP BY sc.id, c.id
           ORDER BY sc.created_at DESC"#,
        category_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();

    let uploads = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           WHERE sub_category_id = ANY($1)"#,
        &ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let result = rows
        .into_iter()
        .map(|r| {
            let sc_uploads = uploads
                .iter()
                .filter(|u| u.sub_category_id == Some(r.id))
                .cloned()
                .map(Upload::with_full_url)
                .collect();

            let mut details = SubCategoryWithDetails::from(r);
            details.uploads = sc_uploads;
            details
        })
        .collect();

    Ok(Json(result))
}

pub async fn create_sub_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateSubCategory>,
) -> Result<(StatusCode, Json<SubCategory>), StatusCode> {
    let slug = slugify(&payload.name, false);

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

    log_audit(
        &state.db,
        Some(claims.sub),
        "create",
        "sub_category",
        Some(sub_category.id),
        "created",
        Some(json!({ "name": sub_category.name, "slug": sub_category.slug })),
    )
    .await;

    Ok((StatusCode::CREATED, Json(sub_category)))
}

pub async fn update_sub_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateSubCategory>,
) -> Result<Json<SubCategory>, StatusCode> {
    let new_slug = payload.name.as_ref().map(|name| slugify(name, false));

    let sub_category = sqlx::query_as!(
        SubCategory,
        r#"UPDATE sub_categories
           SET name = COALESCE($1, name),
               slug = COALESCE($2, slug),
               description = COALESCE($3, description),
               is_active = COALESCE($4, is_active),
               category_id = COALESCE($6, category_id),
               updated_at = NOW()
           WHERE id = $5
           RETURNING id, category_id, name, slug, description, is_active, created_at, updated_at"#,
        payload.name,
        new_slug,
        payload.description,
        payload.is_active,
        uuid,
        payload.category_id,
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    log_audit(
        &state.db,
        Some(claims.sub),
        "update",
        "sub_category",
        Some(sub_category.id),
        "updated",
        Some(json!({
            "name": sub_category.name,
            "slug": sub_category.slug,
            "is_active": sub_category.is_active
        })),
    )
    .await;

    Ok(Json(sub_category))
}

pub async fn delete_sub_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM sub_categories WHERE id = $1", uuid)
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
        "sub_category",
        Some(uuid),
        "deleted",
        None,
    )
    .await;

    Ok(StatusCode::NO_CONTENT)
}
