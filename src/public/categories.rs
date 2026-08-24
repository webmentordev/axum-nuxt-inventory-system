use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicCategoryDetail {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CategoryQuery {
    pub sub_categories: Option<bool>,
    pub is_featured: Option<bool>,
}

#[derive(Debug, Serialize)]
pub struct PublicSubCategoryMini {
    pub name: String,
    pub slug: String,
}

struct CategoryRow {
    name: String,
    slug: String,
    is_featured: bool,
}

struct SubCategoryRow {
    category_slug: String,
    name: String,
    slug: String,
}

#[derive(Debug, Serialize)]
pub struct PublicCategoryWithSub {
    pub name: String,
    pub slug: String,
    pub is_featured: bool,
    pub sub_categories: Option<Vec<PublicSubCategoryMini>>,
}

pub async fn get_public_categories(
    State(state): State<AppState>,
    Query(params): Query<CategoryQuery>,
) -> Result<Json<Vec<PublicCategoryWithSub>>, StatusCode> {
    let only_featured = params.is_featured.unwrap_or(false);

    let categories = if only_featured {
        sqlx::query_as!(
            CategoryRow,
            r#"SELECT name, slug, is_featured
               FROM categories
               WHERE is_active = TRUE AND is_featured = TRUE
               ORDER BY name ASC"#
        )
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        sqlx::query_as!(
            CategoryRow,
            r#"SELECT name, slug, is_featured
               FROM categories
               WHERE is_active = TRUE
               ORDER BY name ASC"#
        )
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    let include_sub = params.sub_categories.unwrap_or(false);

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

            PublicCategoryWithSub {
                name: c.name,
                slug: c.slug,
                is_featured: c.is_featured,
                sub_categories,
            }
        })
        .collect();

    Ok(Json(result))
}

pub async fn get_public_category(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<PublicCategoryDetail>, StatusCode> {
    let category = sqlx::query_as!(
        PublicCategoryDetail,
        r#"SELECT name, slug, description
           FROM categories
           WHERE slug = $1 AND is_active = TRUE"#,
        slug
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(category))
}
