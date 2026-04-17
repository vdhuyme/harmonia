use thiserror::Error;

pub type DomainResult<T> = Result<T, DomainError>;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Validation failed: {0}")]
    ValidationError(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
}
