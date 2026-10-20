use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use indexmap::IndexMap;
use rust_decimal::{Decimal, RoundingStrategy, prelude::ToPrimitive};
use serde::Serialize;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{
    AppState,
    dashboard::uploads::{Upload, WithFullUrl},
};

const SUGGESTED_PRODUCTS_LIMIT: i64 = 4;

pub fn format_money(currency: &str, amount: Decimal, dp: u32) -> String {
    let rounded = amount.round_dp_with_strategy(dp, RoundingStrategy::MidpointAwayFromZero);
    let negative = rounded.is_sign_negative() && !rounded.is_zero();

    let plain = format!("{:.*}", dp as usize, rounded.abs());
    let (int_part, frac_part) = match plain.split_once('.') {
        Some((i, f)) => (i, Some(f)),
        None => (plain.as_str(), None),
    };

    let mut grouped = String::new();
    for (i, ch) in int_part.chars().enumerate() {
        if i > 0 && (int_part.len() - i) % 3 == 0 {
            grouped.push(',');
        }
        grouped.push(ch);
    }

    let mut out = format!(
        "{}{} {}",
        if negative { "-" } else { "" },
        currency,
        grouped
    );
    if let Some(f) = frac_part {
        out.push('.');
        out.push_str(f);
    }
    out
}

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
    pub description: Option<String>,
    pub content: Option<String>,
    pub warranty_months: Option<i16>,
    pub selling_price: Decimal,
    pub compare_at_selling_price: Option<Decimal>,
    pub shipping_cost: Option<Decimal>,
    pub tax: Option<Decimal>,
    pub quantity_in_stock: i32,
    pub image_url: String,
    pub unit: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PublicProductCardRow {
    pub name: String,
    pub slug: String,
    pub brand_id: Option<Uuid>,
    pub model: Option<String>,
    pub selling_price: Decimal,
    pub compare_at_selling_price: Option<Decimal>,
    pub quantity_in_stock: i32,
    pub image_url: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicProductBrand {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub uploads: Vec<Upload>,
}

#[derive(Debug, Serialize)]
pub struct PublicProductCard {
    pub name: String,
    pub slug: String,
    pub brand: Option<PublicProductBrand>,
    pub model: Option<String>,
    pub selling_price_display: String,
    pub compare_at_selling_price_display: Option<String>,
    pub discount_percent: Option<u8>,
    pub in_stock: bool,
    pub image_url: String,
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
    pub description: Option<String>,
    pub content: Option<String>,
    pub warranty_months: Option<i16>,
    pub selling_price: Decimal,
    pub compare_at_selling_price: Option<Decimal>,
    pub selling_price_display: String,
    pub compare_at_selling_price_display: Option<String>,
    pub shipping_cost: Option<Decimal>,
    pub tax: Option<Decimal>,
    pub in_stock: bool,
    pub unit: String,
    pub image_url: String,
    pub uploads: Vec<Upload>,
    pub specifications: IndexMap<String, Vec<PublicSpecification>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_products: Option<Vec<PublicProductCard>>,
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
    pub products: Vec<PublicProductCard>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, serde::Deserialize)]
pub struct GetProductsQuery {
    pub limit: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct RecentlyViewedRequest {
    pub slugs: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct PublicSpecification {
    pub key: String,
    pub value: String,
    pub unit: Option<String>,
    pub is_highlighted: bool,
}

struct SpecificationRow {
    group_name: Option<String>,
    key: String,
    value: String,
    unit: Option<String>,
    is_highlighted: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct PublicProductsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PublicProductsPage {
    pub products: Vec<PublicProductCard>,
    pub has_more: bool,
}

pub fn build_product_card(
    p: PublicProductCardRow,
    brand_map: &HashMap<Uuid, PublicProductBrand>,
    currency: &str,
) -> PublicProductCard {
    let discount_percent = match p.compare_at_selling_price {
        Some(c) if c > p.selling_price && !c.is_zero() => ((c - p.selling_price) / c
            * Decimal::from(100))
        .round()
        .to_u8(),
        _ => None,
    };

    PublicProductCard {
        name: p.name,
        slug: p.slug,
        brand: p.brand_id.and_then(|id| brand_map.get(&id)).cloned(),
        model: p.model,
        selling_price_display: format_money(currency, p.selling_price, 0),
        compare_at_selling_price_display: p
            .compare_at_selling_price
            .filter(|c| *c > p.selling_price)
            .map(|c| format_money(currency, c, 0)),
        discount_percent,
        in_stock: p.quantity_in_stock > 0,
        image_url: p.image_url,
    }
}

pub async fn cards_from_rows(
    state: &AppState,
    rows: Vec<PublicProductCardRow>,
) -> Result<Vec<PublicProductCard>, StatusCode> {
    let brand_ids: Vec<Uuid> = rows.iter().filter_map(|p| p.brand_id).collect();
    let brand_map = fetch_product_brands(state, &brand_ids).await?;

    Ok(rows
        .into_iter()
        .map(|p| build_product_card(p, &brand_map, &state.currency))
        .collect())
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
    currency: &str,
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
        warranty_months: p.warranty_months,
        selling_price: p.selling_price,
        compare_at_selling_price: p.compare_at_selling_price,
        selling_price_display: format_money(currency, p.selling_price, 0),
        compare_at_selling_price_display: p
            .compare_at_selling_price
            .map(|v| format_money(currency, v, 0)),
        in_stock: p.quantity_in_stock > 0,
        shipping_cost: p.shipping_cost,
        tax: p.tax,
        unit: p.unit,
        image_url: p.image_url,
        uploads: product_uploads,
        specifications: IndexMap::new(),
        suggested_products: None,
        seo: None,
    }
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
        PublicProductCardRow,
        r#"SELECT p.name, p.slug, p.brand_id, p.model,
                  p.selling_price, p.compare_at_selling_price, p.quantity_in_stock,
                  COALESCE(p.image_url, '') as "image_url!"
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

    let products = cards_from_rows(&state, rows).await?;

    Ok(Json(SearchProductsResponse {
        products,
        total,
        limit,
        offset,
    }))
}

pub async fn fetch_suggested_products(
    state: &AppState,
    exclude_id: Uuid,
) -> Result<Vec<PublicProductCard>, StatusCode> {
    let rows = sqlx::query_as!(
        PublicProductCardRow,
        r#"SELECT name, slug, brand_id, model,
                  selling_price, compare_at_selling_price, quantity_in_stock,
                  COALESCE(image_url, '') as "image_url!"
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

    cards_from_rows(state, rows).await
}

pub async fn get_public_products(
    State(state): State<AppState>,
    Query(params): Query<PublicProductsQuery>,
) -> Result<Json<PublicProductsPage>, StatusCode> {
    let limit = params.limit.unwrap_or(20).clamp(1, 100);
    let offset = params.offset.unwrap_or(0).max(0);

    let mut rows = sqlx::query_as!(
        PublicProductCardRow,
        r#"SELECT name, slug, brand_id, model,
                  selling_price, compare_at_selling_price, quantity_in_stock,
                  COALESCE(image_url, '') as "image_url!"
           FROM products
           WHERE is_active = TRUE
           ORDER BY created_at DESC, id DESC
           LIMIT $1 OFFSET $2"#,
        limit + 1,
        offset
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let has_more = rows.len() as i64 > limit;
    rows.truncate(limit as usize);

    let products = cards_from_rows(&state, rows).await?;

    Ok(Json(PublicProductsPage { products, has_more }))
}

pub async fn get_public_product(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<PublicProduct>, StatusCode> {
    let p = sqlx::query_as!(
        PublicProductRow,
        r#"SELECT id, name, slug, sku, brand_id, category_id as "category_id!", sub_category_id, model, description, content,
                COALESCE(image_url, '') as "image_url!",
                warranty_months,
                selling_price, compare_at_selling_price, shipping_cost, tax, quantity_in_stock, unit
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

    let spec_rows = sqlx::query_as!(
        SpecificationRow,
        r#"SELECT group_name, key, value, unit, is_highlighted
           FROM product_specifications
           WHERE product_id = $1 AND is_active = TRUE
           ORDER BY sort_order ASC, created_at ASC"#,
        product_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut specifications: IndexMap<String, Vec<PublicSpecification>> = IndexMap::new();
    for row in spec_rows {
        let group = row
            .group_name
            .filter(|g| !g.trim().is_empty())
            .unwrap_or_else(|| "General".to_string());

        specifications
            .entry(group)
            .or_default()
            .push(PublicSpecification {
                key: row.key,
                value: row.value,
                unit: row.unit,
                is_highlighted: row.is_highlighted,
            });
    }

    let mut product = build_public_product(p, &uploads, &brand_map, &state.currency);
    product.category = category;
    product.sub_category = sub_category;
    product.specifications = specifications;
    product.suggested_products = Some(fetch_suggested_products(&state, product_id).await?);
    product.seo = seo;

    Ok(Json(product))
}

pub async fn get_public_products_limited(
    State(state): State<AppState>,
    Query(q): Query<GetProductsQuery>,
) -> Result<Json<Vec<PublicProductCard>>, StatusCode> {
    let limit = q.limit.unwrap_or(20).clamp(1, 100);

    let rows = sqlx::query_as!(
        PublicProductCardRow,
        r#"SELECT name, slug, brand_id, model,
                  selling_price, compare_at_selling_price, quantity_in_stock,
                  COALESCE(image_url, '') as "image_url!"
           FROM products
           WHERE is_active = TRUE
           ORDER BY created_at DESC
           LIMIT $1"#,
        limit
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(cards_from_rows(&state, rows).await?))
}

pub async fn get_recently_viewed_products(
    State(state): State<AppState>,
    Json(req): Json<RecentlyViewedRequest>,
) -> Result<Json<Vec<PublicProductCard>>, StatusCode> {
    let slugs: Vec<String> = req.slugs.into_iter().take(6).collect();

    if slugs.is_empty() {
        return Ok(Json(vec![]));
    }

    let rows = sqlx::query_as!(
        PublicProductCardRow,
        r#"SELECT name, slug, brand_id, model,
                  selling_price, compare_at_selling_price, quantity_in_stock,
                  COALESCE(image_url, '') as "image_url!"
           FROM products
           WHERE slug = ANY($1) AND is_active = TRUE"#,
        &slugs
    )
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut result = cards_from_rows(&state, rows).await?;

    result.sort_by_key(|p| {
        slugs
            .iter()
            .position(|s| *s == p.slug)
            .unwrap_or(usize::MAX)
    });

    Ok(Json(result))
}
