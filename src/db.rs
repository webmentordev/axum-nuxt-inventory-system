use anyhow::{Context, Result};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;

pub async fn connect_database() -> Result<PgPool> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set in .env")?;
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .context("failed to connect to Postgres")?;
    Ok(pool)
}
