mod cache;
mod config;
mod db;
mod domain;
mod error;
mod handlers;
mod repository;

use std::sync::Arc;

use anyhow::Result;
use axum::{
    Json, Router,
    extract::{FromRef, State},
    http::StatusCode,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde_json::{Value, json};
use sqlx::PgPool;
use tracing::info;

use cache::Cache;
use handlers::app::{create_app, get_app, list_apps, update_app};
use repository::app::AppRepository;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    repo: Arc<AppRepository>,
    cache: Arc<Cache>,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(state: &AppState) -> Self {
        state.pool.clone()
    }
}

impl FromRef<AppState> for Arc<AppRepository> {
    fn from_ref(state: &AppState) -> Self {
        state.repo.clone()
    }
}

impl FromRef<AppState> for Arc<Cache> {
    fn from_ref(state: &AppState) -> Self {
        state.cache.clone()
    }
}

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

    let redis_cfg = deadpool_redis::Config::from_url(&config.redis_url);
    let redis_pool = redis_cfg.create_pool(Some(deadpool_redis::Runtime::Tokio1))?;
    info!("redis connection pool established");

    let state = AppState {
        repo: Arc::new(AppRepository::new(pool.clone())),
        cache: Arc::new(Cache::new(redis_pool)),
        pool,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/apps", post(create_app).get(list_apps))
        .route("/apps/{id}", get(get_app).patch(update_app))
        .with_state(state);

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
