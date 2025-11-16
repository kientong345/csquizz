use thiserror::Error;

use crate::domain::error::RepositoryError;

#[derive(Debug, Error)]
pub enum CommentError {
    #[error("Database error: {0}")]
    DbError(#[from] RepositoryError),
}

pub type CommentResult<T> = Result<T, CommentError>;
