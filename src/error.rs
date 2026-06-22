use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    BundleIdConflict,
    InvalidInput(ValidationErrors),
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::NotFound => {
                (StatusCode::NOT_FOUND, Json(json!({"error": "not_found"}))).into_response()
            }
            AppError::BundleIdConflict => (
                StatusCode::CONFLICT,
                Json(json!({"error": "bundle_id_conflict"})),
            )
                .into_response(),
            AppError::InvalidInput(errors) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"error": "validation_failed", "fields": errors})),
            )
                .into_response(),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "internal_server_error"})),
            )
                .into_response(),
        }
    }
}
