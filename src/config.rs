use anyhow::{Context, Result};

pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL must be set")?;
        let redis_url = std::env::var("REDIS_URL").context("REDIS_URL must be set")?;
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .context("PORT must be a valid port number")?;
        Ok(Self {
            database_url,
            redis_url,
            port,
        })
    }
}
