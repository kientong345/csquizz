use thiserror::Error;

use crate::domain::error::RepositoryError;

#[derive(Debug, Error)]
pub enum QuestionError {
    #[error("Database error: {0}")]
    DbError(#[from] RepositoryError),
}

pub type QuestionResult<T> = Result<T, QuestionError>;
