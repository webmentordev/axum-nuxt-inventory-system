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
use crate::dashboard::uploads::*;
use crate::utils::*;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub is_featured: bool,
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
    pub is_featured: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryWithDetails {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub is_active: bool,
    pub is_featured: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub products_count: i64,
    pub sub_categories_count: i64,
    pub uploads: Vec<Upload>,
}

struct CategoryRow {
    id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
    is_active: bool,
    is_featured: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    products_count: i64,
    sub_categories_count: i64,
}

pub async fn get_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<CategoryWithDetails>>, StatusCode> {
    let rows = sqlx::query_as!(
        CategoryRow,
        r#"SELECT c.id, c.name, c.slug, c.description, c.is_active, c.is_featured, c.created_at, c.updated_at,
                  COUNT(DISTINCT p.id) as "products_count!",
                  COUNT(DISTINCT sc.id) as "sub_categories_count!"
           FROM categories c
           LEFT JOIN products p ON p.category_id = c.id
           LEFT JOIN sub_categories sc ON sc.category_id = c.id
           GROUP BY c.id
           ORDER BY c.created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let ids: Vec<Uuid> = rows.iter().map(|r| r.id).collect();

    let uploads = sqlx::query_as!(
    Upload,
    r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
       FROM uploads
       WHERE category_id = ANY($1)"#,
    &ids
)
.fetch_all(&state.db)
.await
.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let categories = rows
        .into_iter()
        .map(|r| {
            let category_uploads = uploads
                .iter()
                .filter(|u| u.category_id == Some(r.id))
                .cloned()
                .map(Upload::with_full_url)
                .collect();

            CategoryWithDetails {
                id: r.id,
                name: r.name,
                slug: r.slug,
                description: r.description,
                is_active: r.is_active,
                is_featured: r.is_featured,
                created_at: r.created_at,
                updated_at: r.updated_at,
                products_count: r.products_count,
                sub_categories_count: r.sub_categories_count,
                uploads: category_uploads,
            }
        })
        .collect();

    Ok(Json(categories))
}

pub async fn get_category(
    State(state): State<AppState>,
    Path(uuid): Path<Uuid>,
) -> Result<Json<CategoryWithDetails>, StatusCode> {
    let row = sqlx::query_as!(
        CategoryRow,
        r#"SELECT c.id, c.name, c.slug, c.description, c.is_active, c.is_featured, c.created_at, c.updated_at,
                  COUNT(DISTINCT p.id) as "products_count!",
                  COUNT(DISTINCT sc.id) as "sub_categories_count!"
           FROM categories c
           LEFT JOIN products p ON p.category_id = c.id
           LEFT JOIN sub_categories sc ON sc.category_id = c.id
           WHERE c.id = $1
           GROUP BY c.id"#,
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
       WHERE category_id = $1"#,
    uuid
)
.fetch_all(&state.db)
.await
.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let uploads = uploads.into_iter().map(Upload::with_full_url).collect();
    Ok(Json(CategoryWithDetails {
        id: row.id,
        name: row.name,
        slug: row.slug,
        description: row.description,
        is_active: row.is_active,
        is_featured: row.is_featured,
        created_at: row.created_at,
        updated_at: row.updated_at,
        products_count: row.products_count,
        sub_categories_count: row.sub_categories_count,
        uploads,
    }))
}

pub async fn create_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateCategory>,
) -> Result<(StatusCode, Json<Category>), StatusCode> {
    let slug = slugify(&payload.name, false);

    let category = sqlx::query_as!(
        Category,
        r#"INSERT INTO categories (name, slug, description)
           VALUES ($1, $2, $3)
           RETURNING id, name, slug, description, is_active, is_featured, created_at, updated_at"#,
        payload.name,
        slug,
        payload.description,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|err| match &err {
        sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
            StatusCode::CONFLICT
        }
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    log_audit(
        &state.db,
        Some(claims.sub),
        "create",
        "category",
        Some(category.id),
        "created",
        Some(json!({ "name": category.name, "slug": category.slug })),
    )
    .await;

    Ok((StatusCode::CREATED, Json(category)))
}

pub async fn update_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(uuid): Path<Uuid>,
    Json(payload): Json<UpdateCategory>,
) -> Result<Json<Category>, StatusCode> {
    let new_slug = payload.name.as_ref().map(|name| slugify(name, false));

    let category = sqlx::query_as!(
        Category,
        r#"UPDATE categories
           SET name = COALESCE($1, name),
               slug = COALESCE($2, slug),
               description = COALESCE($3, description),
               is_active = COALESCE($4, is_active),
               is_featured = COALESCE($5, is_featured),
               updated_at = NOW()
           WHERE id = $6
           RETURNING id, name, slug, description, is_active, is_featured, created_at, updated_at"#,
        payload.name,
        new_slug,
        payload.description,
        payload.is_active,
        payload.is_featured,
        uuid,
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

    log_audit(
        &state.db,
        Some(claims.sub),
        "update",
        "category",
        Some(category.id),
        "updated",
        Some(json!({
            "name": category.name,
            "slug": category.slug,
            "is_active": category.is_active,
            "is_featured": category.is_featured
        })),
    )
    .await;

    Ok(Json(category))
}

pub async fn delete_category(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(uuid): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query!("DELETE FROM categories WHERE id = $1", uuid)
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
        "category",
        Some(uuid),
        "deleted",
        None,
    )
    .await;

    Ok(StatusCode::NO_CONTENT)
}
