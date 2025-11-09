use serde::Serialize;
use sqlx::PgConnection;

use crate::models::error::ModelError;

pub mod paginate;
pub mod post;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

impl Category {
    pub async fn count(connection: &mut PgConnection) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar("SELECT COUNT(*) FROM categories")
            .fetch_one(&mut *connection)
            .await?)
    }

    #[allow(unused)]
    pub async fn count_by_name(
        name: &str,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(
            sqlx::query_scalar("SELECT COUNT(*) FROM categories WHERE name = $1")
                .bind(name)
                .fetch_one(&mut *connection)
                .await?,
        )
    }
}
