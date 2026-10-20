use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{AppState, dashboard::uploads::*, public::products::*};

#[derive(Debug, Serialize)]
pub struct PublicCategoryDetail {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub products: Vec<PublicProductCard>,
    pub has_more: bool,
}

#[derive(Debug, Deserialize)]
pub struct CategoryQuery {
    pub sub_categories: Option<bool>,
    pub is_featured: Option<bool>,
    pub with_uploads: Option<bool>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PublicSubCategoryMini {
    pub name: String,
    pub slug: String,
}

struct CategoryRow {
    id: Uuid,
    name: String,
    slug: String,
    is_featured: bool,
}

struct SubCategoryRow {
    category_slug: String,
    name: String,
    slug: String,
}

struct CategoryDetailRow {
    id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PublicCategoryWithSub {
    pub name: String,
    pub slug: String,
    pub is_featured: bool,
    pub sub_categories: Option<Vec<PublicSubCategoryMini>>,
    pub uploads: Option<Vec<Upload>>,
}

pub async fn get_public_categories(
    State(state): State<AppState>,
    Query(params): Query<CategoryQuery>,
) -> Result<Json<Vec<PublicCategoryWithSub>>, StatusCode> {
    let only_featured = params.is_featured.unwrap_or(false);

    let limit = params.limit.unwrap_or(i64::MAX);

    let categories = if only_featured {
        sqlx::query_as!(
            CategoryRow,
            r#"SELECT id, name, slug, is_featured
               FROM categories
               WHERE is_active = TRUE AND is_featured = TRUE
               ORDER BY name ASC
               LIMIT $1"#,
            limit
        )
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        sqlx::query_as!(
            CategoryRow,
            r#"SELECT id, name, slug, is_featured
               FROM categories
               WHERE is_active = TRUE
               ORDER BY name ASC
               LIMIT $1"#,
            limit
        )
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    let include_sub = params.sub_categories.unwrap_or(false);
    let include_uploads = params.with_uploads.unwrap_or(false);

    let sub_map: Vec<SubCategoryRow> = if include_sub {
        sqlx::query_as!(
            SubCategoryRow,
            r#"SELECT c.slug as category_slug, sc.name, sc.slug
               FROM sub_categories sc
               JOIN categories c ON c.id = sc.category_id
               WHERE sc.is_active = TRUE AND c.is_active = TRUE
               ORDER BY sc.name ASC"#
        )
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        Vec::new()
    };

    let upload_map: Vec<Upload> = if include_uploads {
        let ids: Vec<Uuid> = categories.iter().map(|c| c.id).collect();
        sqlx::query_as!(
            Upload,
            r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
               FROM uploads
               WHERE category_id = ANY($1)"#,
            &ids
        )
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        Vec::new()
    };

    let result = categories
        .into_iter()
        .map(|c| {
            let sub_categories = if include_sub {
                Some(
                    sub_map
                        .iter()
                        .filter(|s| s.category_slug == c.slug)
                        .map(|s| PublicSubCategoryMini {
                            name: s.name.clone(),
                            slug: s.slug.clone(),
                        })
                        .collect(),
                )
            } else {
                None
            };

            let uploads = if include_uploads {
                Some(
                    upload_map
                        .iter()
                        .filter(|u| u.category_id == Some(c.id))
                        .cloned()
                        .map(Upload::with_full_url)
                        .collect(),
                )
            } else {
                None
            };

            PublicCategoryWithSub {
                name: c.name,
                slug: c.slug,
                is_featured: c.is_featured,
                sub_categories,
                uploads,
            }
        })
        .collect();

    Ok(Json(result))
}

pub async fn get_public_category(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(params): Query<PublicProductsQuery>,
) -> Result<Json<PublicCategoryDetail>, StatusCode> {
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = params.offset.unwrap_or(0).max(0);

    let category = sqlx::query_as!(
        CategoryDetailRow,
        r#"SELECT id, name, slug, description
           FROM categories
           WHERE slug = $1 AND is_active = TRUE"#,
        slug
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let mut rows = sqlx::query_as!(
        PublicProductCardRow,
        r#"SELECT name, slug, brand_id, model,
                selling_price, compare_at_selling_price, quantity_in_stock,
                COALESCE(image_url, '') as "image_url!"
        FROM products
        WHERE category_id = $1 AND is_active = TRUE
        ORDER BY created_at DESC, id DESC
        LIMIT $2 OFFSET $3"#,
        category.id,
        limit + 1,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let has_more = rows.len() as i64 > limit;
    rows.truncate(limit as usize);

    let products = cards_from_rows(&state, rows).await?;

    Ok(Json(PublicCategoryDetail {
        name: category.name,
        slug: category.slug,
        description: category.description,
        products,
        has_more,
    }))
}
