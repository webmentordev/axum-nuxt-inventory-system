mod auth;
mod dashboard;
mod db;
mod middleware;
mod public;
mod routes;
mod utils;

use db::*;
use routes::admin::*;
use routes::public::*;

use anyhow::{Context, Result};
use std::net::SocketAddr;

use axum::Json;
use axum::response::IntoResponse;
use axum::{Router, http::StatusCode, routing::get};
use serde_json::json;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub jwt_secret: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let pool = connect_database()
        .await
        .context("could not establish database connection pool")?;
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let state = AppState {
        db: pool,
        jwt_secret,
    };

    let public_routes = init_public_route(state.clone()).await?;
    let admin_routes = init_admin_routes(state.clone()).await?;

    let api_routes = Router::new()
        .nest("/admin", admin_routes)
        .nest("/public", public_routes);

    let app = Router::new()
        .nest("/api", api_routes)
        .nest_service("/uploads", ServeDir::new("uploads"))
        .route("/health", get(health));

    let addr = SocketAddr::from(([0, 0, 0, 0], 7765));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("🚀 Server running at http://127.0.0.1:{}", addr.port());
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({ "message": "System is live!" })),
    )
}
