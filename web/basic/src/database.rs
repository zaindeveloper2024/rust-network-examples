use crate::config;
use sqlx::postgres::PgPoolOptions;
use std::io::Result;

pub async fn setup_database(config: &config::DatabaseConfig) -> Result<sqlx::PgPool> {
    PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(std::time::Duration::from_secs(config.timeout_seconds))
        .connect(&config.url)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}