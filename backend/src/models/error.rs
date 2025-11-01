use thiserror::Error;

#[derive(Error, Debug)]
pub enum ModelError {
    #[error("database error: {0}")]
    Sqlx(#[from] sqlx::Error),

    #[error("jwt error: {0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("bcrypt error: {0}")]
    Bcrypt(#[from] bcrypt::BcryptError),

    #[error("deserialize error: {0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("email already taken: {email}")]
    EmailTaken { email: String },

    #[error("email does not exist: {email}")]
    EmailNotExist { email: String },

    #[error("wrong password for email: {email}")]
    WrongPasswordForEmail { email: String },

    #[error("invalid auth request: {0}")]
    InvalidAuthRequest(String),

    #[error("bad post: {0}")]
    BadPost(String),
}

impl ModelError {
    pub fn get_code(&self) -> u16 {
        match self {
            ModelError::Sqlx(_) => 50001,
            ModelError::Jwt(_) => 50002,
            ModelError::Bcrypt(_) => 50003,
            ModelError::SerdeJson(_) => 50005,
            ModelError::EmailTaken { .. } => 40001,
            ModelError::EmailNotExist { .. } => 40002,
            ModelError::WrongPasswordForEmail { .. } => 40003,
            ModelError::InvalidAuthRequest(_) => 40004,
            ModelError::BadPost(_) => 40005,
        }
    }
}
