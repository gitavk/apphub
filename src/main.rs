mod config;
mod db;
mod domain;
mod error;
mod repository;

use anyhow::Result;
use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use sqlx::PgPool;
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let config = config::Config::from_env()?;

    let pool = db::create_pool(&config.database_url).await?;
    info!("database connection pool established");

    let app = Router::new().route("/health", get(health)).with_state(pool);

    let addr = format!("0.0.0.0:{}", config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("listening on {addr}");
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health(State(pool): State<PgPool>) -> (StatusCode, Json<Value>) {
    match sqlx::query_scalar::<_, DateTime<Utc>>("SELECT NOW()")
        .fetch_one(&pool)
        .await
    {
        Ok(db_time) => (
            StatusCode::OK,
            Json(json!({"status": "ok", "db_time": db_time.to_rfc3339()})),
        ),
        Err(_) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({"status": "unavailable"})),
        ),
    }
}
