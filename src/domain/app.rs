use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct App {
    pub id: Uuid,
    pub bundle_id: String,
    pub name: String,
    pub developer: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}
