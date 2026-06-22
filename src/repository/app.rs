use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::app::App;
use crate::error::AppError;

pub struct CreateApp {
    pub bundle_id: String,
    pub name: String,
    pub developer: String,
    pub description: Option<String>,
}

pub struct UpdateApp {
    pub name: Option<String>,
    pub developer: Option<String>,
    pub description: Option<Option<String>>,
}

pub struct AppRepository {
    pool: PgPool,
}

impl AppRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, input: CreateApp) -> Result<App, AppError> {
        let app = sqlx::query_as!(
            App,
            "INSERT INTO apps (bundle_id, name, developer, description)
             VALUES ($1, $2, $3, $4)
             RETURNING *",
            input.bundle_id,
            input.name,
            input.developer,
            input.description,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            if let sqlx::Error::Database(ref db_err) = e
                && db_err.constraint() == Some("apps_bundle_id_key")
            {
                return AppError::BundleIdConflict;
            }
            AppError::Internal(e.into())
        })?;

        Ok(app)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<App>, AppError> {
        let app = sqlx::query_as!(App, "SELECT * FROM apps WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::Internal(e.into()))?;

        Ok(app)
    }

    pub async fn update(&self, id: Uuid, input: UpdateApp) -> Result<Option<App>, AppError> {
        let app = sqlx::query_as!(
            App,
            "UPDATE apps
             SET name        = COALESCE($2, name),
                 developer   = COALESCE($3, developer),
                 description = COALESCE($4, description),
                 updated_at  = NOW()
             WHERE id = $1
             RETURNING *",
            id,
            input.name,
            input.developer,
            input.description.unwrap_or(None),
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(app)
    }

    pub async fn list(&self, page: u32, per_page: u32) -> Result<Vec<App>, AppError> {
        let offset = (page.saturating_sub(1) * per_page) as i64;
        let limit = per_page as i64;

        let apps = sqlx::query_as!(
            App,
            "SELECT * FROM apps ORDER BY created_at DESC LIMIT $1 OFFSET $2",
            limit,
            offset,
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Internal(e.into()))?;

        Ok(apps)
    }
}
