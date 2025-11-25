use redis::Value;

use crate::{database::non_persistent::SecondaryDatabase, services::error::ServiceError};

pub struct CacheService {
    db: SecondaryDatabase,
}

impl CacheService {
    pub fn init(db: SecondaryDatabase) -> Self {
        Self { db }
    }

    pub async fn get(key: &str) -> Option<Value> {
        None
    }

    pub async fn entry(key: &str, value: Value) -> Result<(), ServiceError> {
        Ok(())
    }

    pub async fn delete(key: &str) -> Result<(), ServiceError> {
        Ok(())
    }
}