use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct DatabaseConfig {
    pub database_url: String,
}

impl DatabaseConfig {
    pub fn get() -> DatabaseConfig {
        #[cfg(feature = "dev")]
        let database_url = std::env::var("DATABASE_URL_DEV").expect("DATABASE_URL_DEV is not set");

        #[cfg(feature = "local")]
        let database_url =
            std::env::var("DATABASE_URL_LOCAL").expect("DATABASE_URL_LOCAL is not set");

        #[cfg(not(any(feature = "dev", feature = "local")))]
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is not set");

        DatabaseConfig { database_url }
    }
}
