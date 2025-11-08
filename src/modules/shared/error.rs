use axum::{Json, http::StatusCode, response::IntoResponse};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Serialize)]
pub struct ErrorResponse<T: Serialize = serde_json::Value> {
    pub message: String,
    pub details: Option<T>,
}

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Payload validation failed")]
    Validation(ValidationErrors),

    #[error("Resource not found")]
    NotFound(String),

    #[error("Conflict")]
    Conflict(String),

    #[error("Unauthorized")]
    Unauthorized(String),

    #[error("Forbidden")]
    Forbidden(String),

    #[error("Internal server error")]
    Internal(anyhow::Error),
}

impl AppError {
    pub fn validation(errs: ValidationErrors) -> Self {
        AppError::Validation(errs)
    }

    pub fn internal<E: Into<anyhow::Error>>(e: E) -> Self {
        AppError::Internal(e.into())
    }

    fn to_response(&self) -> (StatusCode, Json<ErrorResponse>) {
        match self {
            AppError::Validation(errs) => {
                let details = serde_json::to_value(errs).unwrap_or(
                    serde_json::json!({"error": "Failed to serialize validation errors"}),
                );
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(ErrorResponse {
                        message: "Payload validation failed".to_string(),
                        details: Some(details),
                    }),
                )
            }
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    message: msg.clone(),
                    details: None,
                }),
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    message: msg.clone(),
                    details: None,
                }),
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse {
                    message: msg.clone(),
                    details: None,
                }),
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                Json(ErrorResponse {
                    message: msg.clone(),
                    details: None,
                }),
            ),
            AppError::Internal(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    message: "Internal server error".to_string(),
                    details: Some(serde_json::json!({"error": e.to_string()})),
                }),
            ),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, json) = self.to_response();
        (status, json).into_response()
    }
}
