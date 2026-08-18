mod auth;
mod dashboard;
mod db;
mod middleware;
mod routes;
mod utils;

use db::*;
use routes::*;

use anyhow::{Context, Result};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = connect_database()
        .await
        .context("could not establish database connection pool")?;
    let jwt_secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let app = init_route(pool, jwt_secret).await?;

    let addr = SocketAddr::from(([0, 0, 0, 0], 7765));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("🚀 Server running at http://127.0.0.1:{}", addr.port());
    axum::serve(listener, app).await?;
    Ok(())
}
