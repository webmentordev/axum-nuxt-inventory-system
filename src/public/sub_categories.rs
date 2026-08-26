use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    AppState,
    dashboard::images::Image,
    public::products::{
        PublicProduct, PublicProductRow, build_public_product, fetch_product_brands,
    },
};

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicSubCategoryListItem {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct PublicSubCategoryDetail {
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub products: Vec<PublicProduct>,
}

struct SubCategoryDetailRow {
    id: Uuid,
    name: String,
    slug: String,
    description: Option<String>,
}

pub async fn get_public_sub_categories(
    State(state): State<AppState>,
) -> Result<Json<Vec<PublicSubCategoryListItem>>, StatusCode> {
    let sub_categories = sqlx::query_as!(
        PublicSubCategoryListItem,
        r#"SELECT name, slug
           FROM sub_categories
           WHERE is_active = TRUE
           ORDER BY name ASC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(sub_categories))
}

pub async fn get_public_sub_category(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<PublicSubCategoryDetail>, StatusCode> {
    let sub_category = sqlx::query_as!(
        SubCategoryDetailRow,
        r#"SELECT id, name, slug, description
           FROM sub_categories
           WHERE slug = $1 AND is_active = TRUE"#,
        slug
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let products = sqlx::query_as!(
        PublicProductRow,
        r#"SELECT id, name, slug, sku, brand_id, model, description, content, image_url as "image_url!",
                  power_rating_watts, voltage_rating, capacity_ah, warranty_months,
                  selling_price, quantity_in_stock, unit
           FROM products
           WHERE sub_category_id = $1 AND is_active = TRUE
           ORDER BY created_at DESC"#,
        sub_category.id
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

    Ok(Json(PublicSubCategoryDetail {
        name: sub_category.name,
        slug: sub_category.slug,
        description: sub_category.description,
        products,
    }))
}
