use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("deserialize error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("wrong password for email: {email}")]
    WrongPasswordForEmail { email: String },

    #[error("bad post: {0}")]
    BadPost(String),

    #[error("invalid auth schema: {0}")]
    InvalidAuthSchema(String),
}

impl ModelError {
    pub fn get_code(&self) -> u16 {
        match self {
            ModelError::Sqlx(_) => 50001,
            ModelError::SerdeJson(_) => 50005,
            ModelError::WrongPasswordForEmail { .. } => 40003,
            ModelError::BadPost(_) => 40005,
            ModelError::InvalidAuthSchema(_) => 40006,
        }
    }
}
