use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use domain::error::DomainError;

#[derive(Debug)]
pub struct AppError {
    status: StatusCode,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}

impl From<DomainError> for AppError {
    fn from(value: DomainError) -> Self {
        let status = match value {
            DomainError::NotFound(_) => StatusCode::NOT_FOUND,
            DomainError::Conflict(_) => StatusCode::CONFLICT,
            DomainError::ValidationError(_) | DomainError::InvalidInput(_) => {
                StatusCode::BAD_REQUEST
            }
            DomainError::InfrastructureError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };

        Self {
            status,
            message: value.to_string(),
        }
    }
}
