use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct App {
    pub id: Uuid,
    pub bundle_id: String,
    pub name: String,
    pub developer: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}
