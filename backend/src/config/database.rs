use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DatabaseConfig {
    pub database_url: String,
}

impl DatabaseConfig {
    pub fn get() -> DatabaseConfig {
        DatabaseConfig {
            database_url: std::env::var("DATABASE_URL").expect("DATABASE_URL is not set"),
        }
    }
}
