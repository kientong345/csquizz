use std::fs;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Configuration {
    pub port: u16,
}

impl Configuration {
    pub fn get() -> Configuration {
        let content = fs::read_to_string("config.json").expect("cannot get config data");

        serde_json::from_str(&content).expect("cannot get config data")
    }
}

pub fn database_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL is not set")
}

pub fn secret_key() -> Vec<u8> {
    let key = std::env::var("SECRET_KEY").expect("SECRET_KEY is not set");
    key.as_bytes().to_vec()
}
