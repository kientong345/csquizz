use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct AuthConfig {
    pub jwt_secret: String,
}

impl AuthConfig {
    pub fn get() -> AuthConfig {
        AuthConfig {
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET is not set"),
        }
    }
}
