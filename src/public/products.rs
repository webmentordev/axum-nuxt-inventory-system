use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use rust_decimal::Decimal;
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    AppState,
    dashboard::uploads::{Upload, WithFullUrl},
};

const SUGGESTED_PRODUCTS_LIMIT: i64 = 4;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicProductRow {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub sku: String,
    pub brand_id: Option<Uuid>,
    pub category_id: Uuid,
    pub sub_category_id: Option<Uuid>,
    pub model: Option<String>,
    pub product_type: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub power_rating_watts: Option<Decimal>,
    pub per_watt_price: Option<Decimal>,
    pub voltage_rating: Option<Decimal>,
    pub capacity_ah: Option<Decimal>,
    pub warranty_months: Option<i16>,
    pub selling_price: Decimal,
    pub quantity_in_stock: i32,
    pub image_url: String,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicProductBrand {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub uploads: Vec<Upload>,
}

#[derive(Debug, Serialize)]
pub struct PublicProduct {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub sku: String,
    pub brand: Option<PublicProductBrand>,
    pub category: Option<PublicProductCategoryMini>,
    pub sub_category: Option<PublicProductCategoryMini>,
    pub model: Option<String>,
    pub product_type: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub power_rating_watts: Option<Decimal>,
    pub per_watt_price: Option<Decimal>,
    pub voltage_rating: Option<Decimal>,
    pub capacity_ah: Option<Decimal>,
    pub warranty_months: Option<i16>,
    pub selling_price: Decimal,
    pub in_stock: bool,
    pub unit: String,
    pub image_url: String,
    pub uploads: Vec<Upload>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_products: Option<Vec<PublicProduct>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seo: Option<PublicProductSeo>,
}

struct BrandRow {
    id: Uuid,
    name: String,
    slug: String,
}

struct CategoryMiniRow {
    name: String,
    slug: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicProductCategoryMini {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PublicProductSeo {
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image_url: Option<String>,
    pub canonical_url: Option<String>,
    pub focus_keyword: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct SearchProductsRequest {
    pub query: String,
    pub category_id: Option<Uuid>,
    pub sub_category_id: Option<Uuid>,
    pub brand_id: Option<Uuid>,
    pub min_price: Option<Decimal>,
    pub max_price: Option<Decimal>,
    pub in_stock_only: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct SearchProductsResponse {
    pub products: Vec<PublicProduct>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

struct SearchProductRow {
    id: Uuid,
    name: String,
    slug: String,
    sku: String,
    brand_id: Option<Uuid>,
    category_id: Uuid,
    sub_category_id: Option<Uuid>,
    model: Option<String>,
    product_type: Option<String>,
    description: Option<String>,
    content: Option<String>,
    power_rating_watts: Option<Decimal>,
    per_watt_price: Option<Decimal>,
    voltage_rating: Option<Decimal>,
    capacity_ah: Option<Decimal>,
    warranty_months: Option<i16>,
    selling_price: Decimal,
    quantity_in_stock: i32,
    image_url: String,
    unit: String,
}

impl From<SearchProductRow> for PublicProductRow {
    fn from(r: SearchProductRow) -> Self {
        PublicProductRow {
            id: r.id,
            name: r.name,
            slug: r.slug,
            sku: r.sku,
            brand_id: r.brand_id,
            category_id: r.category_id,
            sub_category_id: r.sub_category_id,
            model: r.model,
            product_type: r.product_type,
            description: r.description,
            content: r.content,
            power_rating_watts: r.power_rating_watts,
            per_watt_price: r.per_watt_price,
            voltage_rating: r.voltage_rating,
            capacity_ah: r.capacity_ah,
            warranty_months: r.warranty_months,
            selling_price: r.selling_price,
            quantity_in_stock: r.quantity_in_stock,
            image_url: r.image_url,
            unit: r.unit,
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct GetProductsQuery {
    pub limit: Option<i64>,
}

pub async fn search_public_products(
    State(state): State<AppState>,
    Json(req): Json<SearchProductsRequest>,
) -> Result<Json<SearchProductsResponse>, StatusCode> {
    let query = req.query.trim();
    if query.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let limit = req.limit.unwrap_or(20).clamp(1, 100);
    let offset = req.offset.unwrap_or(0).max(0);
    let min_similarity: f32 = 0.15;

    let rows = sqlx::query_as!(
        SearchProductRow,
        r#"SELECT p.id, p.name, p.slug, p.sku, p.brand_id, p.category_id as "category_id!",
                  p.sub_category_id, p.model, p.product_type, p.description, p.content,
                  p.power_rating_watts, p.per_watt_price, p.voltage_rating, p.capacity_ah,
                  p.warranty_months, p.selling_price, p.quantity_in_stock,
                  p.image_url as "image_url!", p.unit
           FROM products p
           LEFT JOIN brands b ON b.id = p.brand_id
           WHERE p.is_active = TRUE
             AND ($3::uuid IS NULL OR p.category_id = $3)
             AND ($4::uuid IS NULL OR p.sub_category_id = $4)
             AND ($5::uuid IS NULL OR p.brand_id = $5)
             AND ($6::numeric IS NULL OR p.selling_price >= $6)
             AND ($7::numeric IS NULL OR p.selling_price <= $7)
             AND ($8::bool IS NOT TRUE OR p.quantity_in_stock > 0)
             AND (
                p.name ILIKE '%' || $1 || '%'
                OR p.sku ILIKE '%' || $1 || '%'
                OR p.model ILIKE '%' || $1 || '%'
                OR COALESCE(b.name, '') ILIKE '%' || $1 || '%'
                OR similarity(p.name, $1) > $2
                OR similarity(p.sku, $1) > $2
                OR similarity(COALESCE(p.model, ''), $1) > $2
                OR similarity(COALESCE(b.name, ''), $1) > $2
             )
           ORDER BY
                GREATEST(
                    similarity(p.name, $1),
                    similarity(p.sku, $1),
                    similarity(COALESCE(p.model, ''), $1),
                    similarity(COALESCE(b.name, ''), $1)
                ) DESC,
                p.created_at DESC
           LIMIT $9 OFFSET $10"#,
        query,
        min_similarity,
        req.category_id,
        req.sub_category_id,
        req.brand_id,
        req.min_price,
        req.max_price,
        req.in_stock_only,
        limit,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let total = sqlx::query_scalar!(
        r#"SELECT COUNT(*) as "count!"
           FROM products p
           LEFT JOIN brands b ON b.id = p.brand_id
           WHERE p.is_active = TRUE
             AND ($3::uuid IS NULL OR p.category_id = $3)
             AND ($4::uuid IS NULL OR p.sub_category_id = $4)
             AND ($5::uuid IS NULL OR p.brand_id = $5)
             AND ($6::numeric IS NULL OR p.selling_price >= $6)
             AND ($7::numeric IS NULL OR p.selling_price <= $7)
             AND ($8::bool IS NOT TRUE OR p.quantity_in_stock > 0)
             AND (
                p.name ILIKE '%' || $1 || '%'
                OR p.sku ILIKE '%' || $1 || '%'
                OR p.model ILIKE '%' || $1 || '%'
                OR COALESCE(b.name, '') ILIKE '%' || $1 || '%'
                OR similarity(p.name, $1) > $2
                OR similarity(p.sku, $1) > $2
                OR similarity(COALESCE(p.model, ''), $1) > $2
                OR similarity(COALESCE(b.name, ''), $1) > $2
             )"#,
        query,
        min_similarity,
        req.category_id,
        req.sub_category_id,
        req.brand_id,
        req.min_price,
        req.max_price,
        req.in_stock_only,
    )
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let product_ids: Vec<Uuid> = rows.iter().map(|p| p.id).collect();

    let uploads = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           WHERE product_id = ANY($1)
           ORDER BY created_at ASC"#,
        &product_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let brand_ids: Vec<Uuid> = rows.iter().filter_map(|p| p.brand_id).collect();
    let brand_map = fetch_product_brands(&state, &brand_ids).await?;

    let category_ids: Vec<Uuid> = rows.iter().map(|p| p.category_id).collect();
    let sub_category_ids: Vec<Uuid> = rows.iter().filter_map(|p| p.sub_category_id).collect();
    let category_map = fetch_category_minis(&state, &category_ids).await?;
    let sub_category_map = fetch_sub_category_minis(&state, &sub_category_ids).await?;

    let products = rows
        .into_iter()
        .map(|r| {
            let category = category_map.get(&r.category_id).cloned();
            let sub_category = r
                .sub_category_id
                .and_then(|id| sub_category_map.get(&id).cloned());

            let mut product = build_public_product(r.into(), &uploads, &brand_map);
            product.category = category;
            product.sub_category = sub_category;
            product
        })
        .collect();

    Ok(Json(SearchProductsResponse {
        products,
        total,
        limit,
        offset,
    }))
}

pub async fn fetch_product_brands(
    state: &AppState,
    brand_ids: &[Uuid],
) -> Result<HashMap<Uuid, PublicProductBrand>, StatusCode> {
    let brands = sqlx::query_as!(
        BrandRow,
        r#"SELECT id, name, slug
           FROM brands
           WHERE id = ANY($1)"#,
        brand_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uploads = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           WHERE brand_id = ANY($1)"#,
        brand_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let map = brands
        .into_iter()
        .map(|b| {
            let brand_uploads = uploads
                .iter()
                .filter(|u| u.brand_id == Some(b.id))
                .cloned()
                .map(Upload::with_full_url)
                .collect();

            (
                b.id,
                PublicProductBrand {
                    id: b.id,
                    name: b.name,
                    slug: b.slug,
                    uploads: brand_uploads,
                },
            )
        })
        .collect();

    Ok(map)
}

pub fn build_public_product(
    p: PublicProductRow,
    uploads: &[Upload],
    brand_map: &HashMap<Uuid, PublicProductBrand>,
) -> PublicProduct {
    let product_uploads = uploads
        .iter()
        .filter(|u| u.product_id == Some(p.id))
        .cloned()
        .map(Upload::with_full_url)
        .collect();

    let brand = p.brand_id.and_then(|id| brand_map.get(&id)).cloned();

    PublicProduct {
        id: p.id,
        name: p.name,
        slug: p.slug,
        sku: p.sku,
        brand,
        category: None,
        sub_category: None,
        model: p.model,
        description: p.description,
        content: p.content,
        power_rating_watts: p.power_rating_watts,
        product_type: p.product_type,
        per_watt_price: p.per_watt_price,
        voltage_rating: p.voltage_rating,
        capacity_ah: p.capacity_ah,
        warranty_months: p.warranty_months,
        selling_price: p.selling_price,
        in_stock: p.quantity_in_stock > 0,
        unit: p.unit,
        image_url: p.image_url,
        uploads: product_uploads,
        suggested_products: None,
        seo: None,
    }
}

pub async fn fetch_suggested_products(
    state: &AppState,
    exclude_id: Uuid,
) -> Result<Vec<PublicProduct>, StatusCode> {
    let rows = sqlx::query_as!(
        PublicProductRow,
        r#"SELECT id, name, slug, sku, product_type, brand_id, category_id as "category_id!", sub_category_id, model, description, content, image_url as "image_url!",
                  power_rating_watts, per_watt_price, voltage_rating, capacity_ah, warranty_months,
                  selling_price, quantity_in_stock, unit
           FROM products
           WHERE is_active = TRUE AND id != $1
           ORDER BY random()
           LIMIT $2"#,
        exclude_id,
        SUGGESTED_PRODUCTS_LIMIT
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let product_ids: Vec<Uuid> = rows.iter().map(|p| p.id).collect();

    let uploads = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           WHERE product_id = ANY($1)
           ORDER BY created_at ASC"#,
        &product_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let brand_ids: Vec<Uuid> = rows.iter().filter_map(|p| p.brand_id).collect();
    let brand_map = fetch_product_brands(state, &brand_ids).await?;

    Ok(rows
        .into_iter()
        .map(|p| build_public_product(p, &uploads, &brand_map))
        .collect())
}

pub async fn get_public_products(
    State(state): State<AppState>,
) -> Result<Json<Vec<PublicProduct>>, StatusCode> {
    let products = sqlx::query_as!(
        PublicProductRow,
        r#"SELECT id, name, slug, sku, product_type, brand_id, category_id as "category_id!", sub_category_id, model, description, content, image_url as "image_url!",
                  power_rating_watts, per_watt_price, voltage_rating, capacity_ah, warranty_months,
                  selling_price, quantity_in_stock, unit
           FROM products
           WHERE is_active = TRUE
           ORDER BY created_at DESC"#
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let product_ids: Vec<Uuid> = products.iter().map(|p| p.id).collect();

    let uploads = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           WHERE product_id = ANY($1)
           ORDER BY created_at ASC"#,
        &product_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let brand_ids: Vec<Uuid> = products.iter().filter_map(|p| p.brand_id).collect();
    let brand_map = fetch_product_brands(&state, &brand_ids).await?;

    let category_ids: Vec<Uuid> = products.iter().map(|p| p.category_id).collect();
    let sub_category_ids: Vec<Uuid> = products.iter().filter_map(|p| p.sub_category_id).collect();

    let category_map = fetch_category_minis(&state, &category_ids).await?;
    let sub_category_map = fetch_sub_category_minis(&state, &sub_category_ids).await?;

    let result = products
        .into_iter()
        .map(|p| {
            let category = category_map.get(&p.category_id).cloned();
            let sub_category = p
                .sub_category_id
                .and_then(|id| sub_category_map.get(&id).cloned());

            let mut product = build_public_product(p, &uploads, &brand_map);
            product.category = category;
            product.sub_category = sub_category;
            product
        })
        .collect();

    Ok(Json(result))
}

pub async fn fetch_category_minis(
    state: &AppState,
    ids: &[Uuid],
) -> Result<HashMap<Uuid, PublicProductCategoryMini>, StatusCode> {
    struct Row {
        id: Uuid,
        name: String,
        slug: String,
    }

    let rows = sqlx::query_as!(
        Row,
        r#"SELECT id, name, slug FROM categories WHERE id = ANY($1)"#,
        ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(rows
        .into_iter()
        .map(|r| {
            (
                r.id,
                PublicProductCategoryMini {
                    name: r.name,
                    slug: r.slug,
                },
            )
        })
        .collect())
}

pub async fn fetch_sub_category_minis(
    state: &AppState,
    ids: &[Uuid],
) -> Result<HashMap<Uuid, PublicProductCategoryMini>, StatusCode> {
    struct Row {
        id: Uuid,
        name: String,
        slug: String,
    }

    let rows = sqlx::query_as!(
        Row,
        r#"SELECT id, name, slug FROM sub_categories WHERE id = ANY($1)"#,
        ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(rows
        .into_iter()
        .map(|r| {
            (
                r.id,
                PublicProductCategoryMini {
                    name: r.name,
                    slug: r.slug,
                },
            )
        })
        .collect())
}

pub async fn get_public_product(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<PublicProduct>, StatusCode> {
    let p = sqlx::query_as!(
        PublicProductRow,
        r#"SELECT id, name, slug, sku, product_type, brand_id, category_id as "category_id!", sub_category_id, model, description, content, image_url as "image_url!",
                  power_rating_watts, per_watt_price, voltage_rating, capacity_ah, warranty_months,
                  selling_price, quantity_in_stock, unit
           FROM products
           WHERE slug = $1 AND is_active = TRUE"#,
        slug
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .ok_or(StatusCode::NOT_FOUND)?;

    let seo = sqlx::query_as!(
        PublicProductSeo,
        r#"SELECT meta_title, meta_description, meta_keywords, og_title, og_description, og_image_url, canonical_url, focus_keyword
           FROM products_seo
           WHERE product_id = $1"#,
        p.id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let uploads = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           WHERE product_id = $1
           ORDER BY created_at ASC"#,
        p.id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let brand_ids: Vec<Uuid> = p.brand_id.into_iter().collect();
    let brand_map = fetch_product_brands(&state, &brand_ids).await?;

    let category = sqlx::query_as!(
        CategoryMiniRow,
        r#"SELECT name, slug FROM categories WHERE id = $1"#,
        p.category_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    .map(|c| PublicProductCategoryMini {
        name: c.name,
        slug: c.slug,
    });

    let sub_category = if let Some(sub_category_id) = p.sub_category_id {
        sqlx::query_as!(
            CategoryMiniRow,
            r#"SELECT name, slug FROM sub_categories WHERE id = $1"#,
            sub_category_id
        )
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .map(|c| PublicProductCategoryMini {
            name: c.name,
            slug: c.slug,
        })
    } else {
        None
    };

    let product_id = p.id;
    let mut product = build_public_product(p, &uploads, &brand_map);
    product.category = category;
    product.sub_category = sub_category;
    product.suggested_products = Some(fetch_suggested_products(&state, product_id).await?);
    product.seo = seo;

    Ok(Json(product))
}

pub async fn get_public_products_limited(
    State(state): State<AppState>,
    Query(q): Query<GetProductsQuery>,
) -> Result<Json<Vec<PublicProduct>>, StatusCode> {
    let limit = q.limit.unwrap_or(20).clamp(1, 100);

    let products = sqlx::query_as!(
        PublicProductRow,
        r#"SELECT id, name, slug, sku, product_type, brand_id, category_id as "category_id!", sub_category_id, model, description, content, image_url as "image_url!",
                  power_rating_watts, per_watt_price, voltage_rating, capacity_ah, warranty_months,
                  selling_price, quantity_in_stock, unit
           FROM products
           WHERE is_active = TRUE
           ORDER BY created_at DESC
           LIMIT $1"#,
        limit
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let product_ids: Vec<Uuid> = products.iter().map(|p| p.id).collect();

    let uploads = sqlx::query_as!(
        Upload,
        r#"SELECT id, product_id, category_id, sub_category_id, brand_id, name, file_path, file_type, created_at
           FROM uploads
           WHERE product_id = ANY($1)
           ORDER BY created_at ASC"#,
        &product_ids
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let brand_ids: Vec<Uuid> = products.iter().filter_map(|p| p.brand_id).collect();
    let brand_map = fetch_product_brands(&state, &brand_ids).await?;

    let category_ids: Vec<Uuid> = products.iter().map(|p| p.category_id).collect();
    let sub_category_ids: Vec<Uuid> = products.iter().filter_map(|p| p.sub_category_id).collect();

    let category_map = fetch_category_minis(&state, &category_ids).await?;
    let sub_category_map = fetch_sub_category_minis(&state, &sub_category_ids).await?;

    let result = products
        .into_iter()
        .map(|p| {
            let category = category_map.get(&p.category_id).cloned();
            let sub_category = p
                .sub_category_id
                .and_then(|id| sub_category_map.get(&id).cloned());

            let mut product = build_public_product(p, &uploads, &brand_map);
            product.category = category;
            product.sub_category = sub_category;
            product
        })
        .collect();

    Ok(Json(result))
}
