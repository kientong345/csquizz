use serde::Deserialize;
use sqlx::PgConnection;

use crate::models::category::QuizCategory;

#[derive(Debug, Deserialize)]
pub struct PostQuizCategory {
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

impl QuizCategory {
    pub async fn create(
        data: PostQuizCategory,
        connection: &mut PgConnection,
    ) -> Result<QuizCategory, sqlx::Error> {
        Ok(sqlx::query_as!(
            QuizCategory,
            r#"INSERT INTO categories (name, image_url, description)
            VALUES ($1, $2, $3)
            RETURNING id, name, image_url, description"#,
            data.name,
            data.image_url,
            data.description
        )
        .fetch_one(connection)
        .await?)
    }
}
