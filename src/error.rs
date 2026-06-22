use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    BundleIdConflict,
    Internal(anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, code) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "not_found"),
            AppError::BundleIdConflict => (StatusCode::CONFLICT, "bundle_id_conflict"),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_server_error"),
        };
        (status, Json(json!({"error": code}))).into_response()
    }
}
