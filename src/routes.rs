use crate::auth::*;
use crate::dashboard::barcodes::*;
use crate::dashboard::categories::*;
use crate::dashboard::images::*;
use crate::dashboard::orders::*;
use crate::dashboard::products::*;
use crate::dashboard::sub_categories::*;

use anyhow::Result;
use axum::Json;
use axum::response::IntoResponse;
use axum::{Router, extract::State, http::StatusCode, routing::get};
use serde_json::json;
use sqlx::postgres::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

pub async fn init_route(pool: PgPool) -> Result<Router> {
    let state = AppState { db: pool };

    let product = Router::new()
        .route("/", get(get_products).post(create_product))
        .route(
            "/product/{uuid}",
            get(get_product)
                .patch(update_product)
                .delete(delete_product),
        );
    let categories = Router::new()
        .route("/", get(get_categories).post(create_category))
        .route(
            "/category/{uuid}",
            get(get_category)
                .patch(update_category)
                .delete(delete_category),
        );
    let sub_categories = Router::new()
        .route("/", get(get_sub_categories).post(create_sub_category))
        .route(
            "/sub-category/{uuid}",
            get(get_sub_category)
                .patch(update_sub_category)
                .delete(delete_sub_category),
        );
    let orders = Router::new()
        .route("/", get(get_orders).post(create_order))
        .route(
            "/order/{uuid}",
            get(get_order).patch(update_order).delete(delete_order),
        );
    let images = Router::new()
        .route("/", get(get_images).post(create_image))
        .route(
            "/image/{uuid}",
            get(get_image).patch(update_image).delete(delete_image),
        );
    let barcodes = Router::new()
        .route("/", get(get_barcodes).post(create_barcode))
        .route(
            "/barcode/{uuid}",
            get(get_barcode)
                .patch(update_barcode)
                .delete(delete_barcode),
        );

    let api_routes = Router::new()
        .nest("/products", product)
        .nest("/categories", categories)
        .nest("/sub-categories", sub_categories)
        .nest("/orders", orders)
        .nest("/images", images)
        .nest("/barcodes", barcodes);

    let rotues = Router::new()
        .nest("/api", api_routes)
        .route("/health", get(health))
        .with_state(state);
    Ok(rotues)
}

async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "message": "System is live!" })),
    )
}
