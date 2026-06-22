use deadpool_redis::Pool;
use deadpool_redis::redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};
use tracing::warn;

pub struct Cache {
    pool: Pool,
}

impl Cache {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let mut conn = match self.pool.get().await {
            Ok(c) => c,
            Err(e) => {
                warn!("cache pool error on get: {e}");
                return None;
            }
        };
        let raw: Option<String> = match conn.get(key).await {
            Ok(v) => v,
            Err(e) => {
                warn!("cache get error for key={key}: {e}");
                return None;
            }
        };
        raw.and_then(|s| match serde_json::from_str(&s) {
            Ok(v) => Some(v),
            Err(e) => {
                warn!("cache deserialize error for key={key}: {e}");
                None
            }
        })
    }

    pub async fn set_ex<T: Serialize>(&self, key: &str, value: &T, ttl_secs: u64) {
        let json = match serde_json::to_string(value) {
            Ok(s) => s,
            Err(e) => {
                warn!("cache serialize error for key={key}: {e}");
                return;
            }
        };
        let mut conn = match self.pool.get().await {
            Ok(c) => c,
            Err(e) => {
                warn!("cache pool error on set_ex: {e}");
                return;
            }
        };
        if let Err(e) = conn.set_ex::<_, _, ()>(key, json, ttl_secs).await {
            warn!("cache set_ex error for key={key}: {e}");
        }
    }

    pub async fn del(&self, key: &str) {
        let mut conn = match self.pool.get().await {
            Ok(c) => c,
            Err(e) => {
                warn!("cache pool error on del: {e}");
                return;
            }
        };
        if let Err(e) = conn.del::<_, ()>(key).await {
            warn!("cache del error for key={key}: {e}");
        }
    }
}
