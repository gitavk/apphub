use anyhow::Result;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn create_pool(url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await?;
    Ok(pool)
}
