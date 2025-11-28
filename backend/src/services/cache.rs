use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};

use crate::{database::non_persistent::SecondaryDatabase, services::error::ServiceError};

#[derive(Clone)]
pub struct CacheService {
    db: SecondaryDatabase,
}

impl CacheService {
    pub fn init(db: SecondaryDatabase) -> Self {
        Self { db }
    }

    pub async fn get_key<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, ServiceError> {
        let mut connection = self.db.get_connection().await?;
        let retrieved_data: Option<String> = connection.get(key).await?;
        if let Some(data) = retrieved_data {
            let value = serde_json::from_str(data.as_str()).ok();
            Ok(value)
        } else {
            Ok(None)
        }
    }

    pub async fn set_key<T: Serialize>(&self, key: &str, value: &T) -> Result<(), ServiceError> {
        let mut connection = self.db.get_connection().await?;
        let serialized_value = serde_json::to_string(value).ok();
        if let Some(value) = serialized_value {
            let _: () = connection.set(key, value).await?;
        }
        Ok(())
    }

    pub async fn delete_key(&self, key: &str) -> Result<(), ServiceError> {
        let mut connection = self.db.get_connection().await?;
        let _: () = connection.del(key).await.unwrap();
        Ok(())
    }
}
