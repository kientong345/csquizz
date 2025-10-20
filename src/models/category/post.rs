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

#[cfg(test)]
mod tests {
    use sqlx::{pool::PoolConnection, Postgres};

    use crate::models::category::{post::PostQuizCategory, QuizCategory};

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_new_category(mut conn: PoolConnection<Postgres>) {
        let data = PostQuizCategory {
            name: String::from("Operating System"),
            image_url: Some(String::from("https://www.telecomreviewafrica.com/wp-content/uploads/2019/12/Operating_systemsThe_heart_of_smartphones_intro.jpg")),
            description: Some(String::from("An operating system (OS) is system software that manages computer hardware and software resources, and provides common services for computer programs.")),
        };

        let category = QuizCategory::create(data, &mut conn).await.unwrap();

        assert_eq!(category.name, "Operating System".to_string());
        assert_eq!(category.image_url, Some(String::from("https://www.telecomreviewafrica.com/wp-content/uploads/2019/12/Operating_systemsThe_heart_of_smartphones_intro.jpg")));
        assert_eq!(category.description, Some(String::from("An operating system (OS) is system software that manages computer hardware and software resources, and provides common services for computer programs.")));
    }
}
