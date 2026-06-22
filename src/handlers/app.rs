use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
    cache::Cache,
    domain::app::App,
    error::AppError,
    repository::app::{AppRepository, CreateApp, UpdateApp},
};

#[derive(Deserialize, Validate)]
pub struct CreateAppRequest {
    #[validate(length(min = 1, max = 255))]
    pub bundle_id: String,
    #[validate(length(min = 1))]
    pub name: String,
    #[validate(length(min = 1))]
    pub developer: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct ListParams {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

pub async fn create_app(
    State(repo): State<Arc<AppRepository>>,
    Json(body): Json<CreateAppRequest>,
) -> Result<(StatusCode, Json<App>), AppError> {
    body.validate().map_err(AppError::InvalidInput)?;

    let app = repo
        .create(CreateApp {
            bundle_id: body.bundle_id,
            name: body.name,
            developer: body.developer,
            description: body.description,
        })
        .await?;

    Ok((StatusCode::CREATED, Json(app)))
}

pub async fn list_apps(
    State(repo): State<Arc<AppRepository>>,
    State(cache): State<Arc<Cache>>,
    Query(params): Query<ListParams>,
) -> Result<Json<Vec<App>>, AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).clamp(1, 100);
    let key = format!("apps:v1:{page}:{per_page}");

    if let Some(apps) = cache.get::<Vec<App>>(&key).await {
        return Ok(Json(apps));
    }

    let apps = repo.list(page, per_page).await?;
    cache.set_ex(&key, &apps, 30).await;
    Ok(Json(apps))
}

#[derive(Deserialize, Validate)]
pub struct UpdateAppRequest {
    #[validate(length(min = 1))]
    pub name: Option<String>,
    #[validate(length(min = 1))]
    pub developer: Option<String>,
    pub description: Option<Option<String>>,
}

pub async fn update_app(
    State(repo): State<Arc<AppRepository>>,
    State(cache): State<Arc<Cache>>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateAppRequest>,
) -> Result<Json<App>, AppError> {
    body.validate().map_err(AppError::InvalidInput)?;

    let app = repo
        .update(
            id,
            UpdateApp {
                name: body.name,
                developer: body.developer,
                description: body.description,
            },
        )
        .await?
        .ok_or(AppError::NotFound)?;

    cache.del(&format!("app:v1:{id}")).await;
    Ok(Json(app))
}

pub async fn get_app(
    State(repo): State<Arc<AppRepository>>,
    State(cache): State<Arc<Cache>>,
    Path(id): Path<Uuid>,
) -> Result<Json<App>, AppError> {
    let key = format!("app:v1:{id}");

    if let Some(app) = cache.get::<App>(&key).await {
        return Ok(Json(app));
    }

    match repo.find_by_id(id).await? {
        Some(app) => {
            cache.set_ex(&key, &app, 300).await;
            Ok(Json(app))
        }
        None => Err(AppError::NotFound),
    }
}
