use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use domain::AppError as DomainError;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized,
    InternalError(String),
    ValidationError(String),
    // Auth errors
    MissingAuthToken,
    InvalidAuthToken,
    InvalidToken,
    TokenExpired,
    TokenCreationFailed,
    // Crypto errors
    EncryptionFailed,
    DecryptionFailed,
}

impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::NotFound(msg) => AppError::NotFound(msg),
            DomainError::Unauthorized => AppError::Unauthorized,
            DomainError::ValidationFailed(msg) => {
                AppError::ValidationError(msg)
            }
            DomainError::DatabaseError(msg) => AppError::InternalError(msg),
            DomainError::ProviderError(msg) => AppError::InternalError(msg),
            DomainError::InternalError(msg) => AppError::InternalError(msg),
            DomainError::UnknownProvider => {
                AppError::BadRequest("Unknown provider".to_string())
            }
            DomainError::InvalidStatus => {
                AppError::BadRequest("Invalid status".to_string())
            }
            DomainError::DuplicateVote => {
                AppError::ValidationError("Duplicate vote".to_string())
            }
            DomainError::RedisError(msg) => AppError::InternalError(msg),
            DomainError::LockAcquisitionFailed => {
                AppError::InternalError("Queue lock unavailable".to_string())
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error, details) = match self {
            AppError::NotFound(msg) => {
                (StatusCode::NOT_FOUND, "Not found", Some(msg))
            }
            AppError::BadRequest(msg) => {
                (StatusCode::BAD_REQUEST, "Bad request", Some(msg))
            }
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Unauthorized", None)
            }
            AppError::ValidationError(msg) => {
                (StatusCode::BAD_REQUEST, "Validation error", Some(msg))
            }
            AppError::InternalError(msg) => {
                tracing::error!("Internal error: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error",
                    None,
                )
            }
            AppError::MissingAuthToken => (
                StatusCode::UNAUTHORIZED,
                "Missing authentication token",
                None,
            ),
            AppError::InvalidAuthToken => (
                StatusCode::UNAUTHORIZED,
                "Invalid authentication token format",
                None,
            ),
            AppError::InvalidToken => {
                (StatusCode::UNAUTHORIZED, "Invalid or malformed token", None)
            }
            AppError::TokenExpired => {
                (StatusCode::UNAUTHORIZED, "Token has expired", None)
            }
            AppError::TokenCreationFailed => {
                tracing::error!("Failed to create token");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create authentication token",
                    None,
                )
            }
            AppError::EncryptionFailed => {
                tracing::error!("Encryption failed");
                (StatusCode::INTERNAL_SERVER_ERROR, "Encryption failed", None)
            }
            AppError::DecryptionFailed => {
                tracing::error!("Decryption failed");
                (StatusCode::INTERNAL_SERVER_ERROR, "Decryption failed", None)
            }
        };

        let body = Json(ErrorResponse {
            error: error.to_string(),
            details,
        });

        (status, body).into_response()
    }
}
