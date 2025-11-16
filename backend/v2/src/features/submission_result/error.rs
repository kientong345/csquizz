use thiserror::Error;

use crate::domain::error::RepositoryError;

#[derive(Debug, Error)]
pub enum SubmissionError {
    #[error("Database error: {0}")]
    DbError(#[from] RepositoryError),
}

pub type SubmissionResult<T> = Result<T, SubmissionError>;
