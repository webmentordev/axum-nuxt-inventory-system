use crate::auth::*;
use crate::dashboard::barcodes::*;
use crate::dashboard::categories::*;
use crate::dashboard::contacts::*;
use crate::dashboard::images::*;
use crate::dashboard::orders::*;
use crate::dashboard::products::*;
use crate::dashboard::sub_categories::*;
use crate::middleware::require_admin;
use axum::middleware as axum_middleware;

use anyhow::Result;
use axum::Json;
use axum::response::IntoResponse;
use axum::{
    Router,
    http::StatusCode,
    routing::{delete, get, post},
};
use serde_json::json;
use sqlx::postgres::PgPool;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub jwt_secret: String,
}

pub async fn init_route(pool: PgPool, jwt_secret: String) -> Result<Router> {
    let state = AppState {
        db: pool,
        jwt_secret,
    };

    let product = Router::new()
        .route("/", get(get_products).post(create_product))
        .route(
            "/{uuid}",
            get(get_product)
                .patch(update_product)
                .delete(delete_product),
        );
    let categories = Router::new()
        .route("/", get(get_categories).post(create_category))
        .route(
            "/{uuid}",
            get(get_category)
                .patch(update_category)
                .delete(delete_category),
        );
    let sub_categories = Router::new()
        .route("/", get(get_sub_categories).post(create_sub_category))
        .route(
            "/{uuid}",
            get(get_sub_category)
                .patch(update_sub_category)
                .delete(delete_sub_category),
        );
    let orders = Router::new()
        .route("/", get(get_orders).post(create_order))
        .route(
            "/{uuid}",
            get(get_order).patch(update_order).delete(delete_order),
        );
    let images = Router::new()
        .route("/", get(get_images).post(create_image))
        .route(
            "/{uuid}",
            get(get_image).patch(update_image).delete(delete_image),
        );
    let barcodes = Router::new()
        .route("/", get(get_barcodes).post(create_barcode))
        .route("/bulk", post(create_barcodes_bulk))
        .route(
            "/{uuid}",
            get(get_barcode)
                .patch(update_barcode)
                .delete(delete_barcode),
        );

    let users_protected = Router::new().route("/", get(get_users)).route(
        "/{uuid}",
        get(get_user).patch(update_user).delete(delete_user),
    );

    let users_public = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user));

    let contacts_public = Router::new().route("/", post(create_contact));

    let contacts_protected = Router::new()
        .route("/", get(get_contacts))
        .route("/{uuid}", delete(delete_contact));

    let api_routes = Router::new()
        .nest("/products", product)
        .nest("/categories", categories)
        .nest("/sub-categories", sub_categories)
        .nest("/orders", orders)
        .nest("/images", images)
        .nest("/barcodes", barcodes)
        .nest("/users", users_protected)
        .nest("/contacts", contacts_protected)
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            require_admin,
        ));

    let routes = Router::new()
        .nest("/api", api_routes)
        .nest("/api/users", users_public)
        .nest("/api/contacts", contacts_public)
        .nest_service("/uploads", ServeDir::new("uploads"))
        .route("/health", get(health))
        .with_state(state);
    Ok(routes)
}

async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "message": "System is live!" })),
    )
}
