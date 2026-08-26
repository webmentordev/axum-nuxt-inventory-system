use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    AppState,
    dashboard::images::Image,
    public::products::{
        PublicProduct, PublicProductRow, build_public_product, fetch_product_brands,
    },
};

#[derive(Debug, Serialize)]
pub struct PublicCategoryDetail {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub products: Vec<PublicProduct>,
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

    let products = sqlx::query_as!(
        PublicProductRow,
        r#"SELECT id, name, slug, sku, brand_id, model, description, image_url as "image_url!",
                  power_rating_watts, voltage_rating, capacity_ah, warranty_months,
                  selling_price, quantity_in_stock, unit
           FROM products
           WHERE category_id = $1 AND is_active = TRUE
           ORDER BY created_at DESC"#,
        category.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let product_ids: Vec<Uuid> = products.iter().map(|p| p.id).collect();

    let images = sqlx::query_as!(
        Image,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, created_at
           FROM images
           WHERE product_id = ANY($1)
           ORDER BY created_at ASC"#,
        &product_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let brand_ids: Vec<Uuid> = products.iter().filter_map(|p| p.brand_id).collect();
    let brand_map = fetch_product_brands(&state, &brand_ids).await?;

    let products = products
        .into_iter()
        .map(|p| build_public_product(p, &images, &brand_map))
        .collect();

    Ok(Json(PublicCategoryDetail {
        name: category.name,
        slug: category.slug,
        description: category.description,
        products,
    }))
}
