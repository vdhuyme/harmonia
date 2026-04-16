use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Validation failed: {0}")]
    ValidationFailed(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Provider error: {0}")]
    ProviderError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("Unknown provider")]
    UnknownProvider,

    #[error("Invalid status")]
    InvalidStatus,

    #[error("Duplicate vote")]
    DuplicateVote,

    #[error("Redis error: {0}")]
    RedisError(String),

    #[error("Lock acquisition failed")]
    LockAcquisitionFailed,
}

pub type Result<T> = std::result::Result<T, AppError>;
