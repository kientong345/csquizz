use thiserror::Error;

use crate::domain::error::RepositoryError;

#[derive(Debug, Error)]
pub enum CategoryError {
    #[error("Database error: {0}")]
    DbError(#[from] RepositoryError),
}

pub type CategoryResult<T> = Result<T, CategoryError>;
