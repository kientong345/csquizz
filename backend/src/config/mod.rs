use serde::{Deserialize, Serialize};

use crate::config::{
    app::AppConfig, auth::AuthConfig, database::DatabaseConfig, oauth::OAuthConfig,
};

pub mod app;
pub mod auth;
pub mod database;
pub mod oauth;

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Configuration {
    pub app_config: AppConfig,
    pub db_config: DatabaseConfig,
    pub auth_config: AuthConfig,
    pub oauth_config: OAuthConfig,
}

impl Configuration {
    pub fn get() -> Configuration {
        Configuration {
            app_config: AppConfig::get(),
            db_config: DatabaseConfig::get(),
            auth_config: AuthConfig::get(),
            oauth_config: OAuthConfig::get(),
        }
    }
}
