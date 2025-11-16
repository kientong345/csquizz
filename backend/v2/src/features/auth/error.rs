use thiserror::Error;

use crate::domain::error::RepositoryError;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Database error: {0}")]
    DbError(#[from] RepositoryError),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String), // e.g., User already exists

    #[error("Internal server error")]
    Internal,
}

pub type AuthResult<T> = Result<T, AuthError>;
