use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection};

pub mod paginate;
pub mod post;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct QuizCategory {
    pub id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

impl QuizCategory {
    pub async fn count(connection: &mut PgConnection) -> Result<i64, sqlx::Error> {
        Ok(sqlx::query_scalar("SELECT COUNT(*) FROM categories")
            .fetch_one(&mut *connection)
            .await?)
    }

    #[allow(unused)]
    pub async fn count_by_name(
        name: &str,
        connection: &mut PgConnection,
    ) -> Result<i64, sqlx::Error> {
        Ok(
            sqlx::query_scalar("SELECT COUNT(*) FROM categories WHERE name = $1")
                .bind(name)
                .fetch_one(&mut *connection)
                .await?,
        )
    }
}
