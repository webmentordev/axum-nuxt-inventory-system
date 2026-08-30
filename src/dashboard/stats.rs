use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;

use crate::AppState;

#[derive(Debug, Serialize)]
pub struct DashboardStats {
    pub uploads_count: i64,
    pub products_count: i64,
    pub orders_count: i64,
    pub categories_count: i64,
    pub sub_categories_count: i64,
    pub contacts_count: i64,
    pub brands_count: i64,
    pub barcodes_count: i64,
    pub users_count: i64,
    pub admin_users_count: i64,
}

pub async fn get_dashboard_stats(
    State(state): State<AppState>,
) -> Result<Json<DashboardStats>, StatusCode> {
    let uploads_count = sqlx::query_scalar!("SELECT COUNT(*) FROM uploads")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0);

    let products_count = sqlx::query_scalar!("SELECT COUNT(*) FROM products")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0);

    let orders_count = sqlx::query_scalar!("SELECT COUNT(*) FROM orders")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0);

    let categories_count = sqlx::query_scalar!("SELECT COUNT(*) FROM categories")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0);

    let sub_categories_count = sqlx::query_scalar!("SELECT COUNT(*) FROM sub_categories")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0);

    let contacts_count = sqlx::query_scalar!("SELECT COUNT(*) FROM contacts")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0);

    let brands_count = sqlx::query_scalar!("SELECT COUNT(*) FROM brands")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0);

    let barcodes_count = sqlx::query_scalar!("SELECT COUNT(*) FROM barcodes")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0);

    let users_count = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0);

    let admin_users_count = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE is_admin = TRUE")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .unwrap_or(0);

    Ok(Json(DashboardStats {
        uploads_count,
        products_count,
        orders_count,
        categories_count,
        sub_categories_count,
        contacts_count,
        brands_count,
        barcodes_count,
        users_count,
        admin_users_count,
    }))
}
