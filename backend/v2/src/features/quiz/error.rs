use thiserror::Error;

use crate::domain::error::RepositoryError;

#[derive(Debug, Error)]
pub enum QuizError {
    #[error("Database error: {0}")]
    DbError(#[from] RepositoryError),
}

pub type QuizResult<T> = Result<T, QuizError>;
