use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("Not found")]
    NotFound,

    #[error("Unknown error")]
    Unknown,
}

pub type RepositoryResult<T> = Result<T, RepositoryError>;
