use serde::Deserialize;
use sqlx::PgConnection;

use crate::models::{category::Category, error::ModelError};

#[derive(Debug, Deserialize)]
pub struct PostCategory {
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

impl Category {
    pub async fn create_from(
        data: PostCategory,
        connection: &mut PgConnection,
    ) -> Result<Category, ModelError> {
        Ok(sqlx::query_as!(
            Category,
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

    use crate::models::category::{post::PostCategory, Category};

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_new_category(mut conn: PoolConnection<Postgres>) {
        let data = PostCategory {
            name: String::from("Operating System"),
            image_url: Some(String::from("https://www.telecomreviewafrica.com/wp-content/uploads/2019/12/Operating_systemsThe_heart_of_smartphones_intro.jpg")),
            description: Some(String::from("An operating system (OS) is system software that manages computer hardware and software resources, and provides common services for computer programs.")),
        };

        let category = Category::create_from(data, &mut conn).await.unwrap();

        assert_eq!(category.name, "Operating System".to_string());
        assert_eq!(category.image_url, Some(String::from("https://www.telecomreviewafrica.com/wp-content/uploads/2019/12/Operating_systemsThe_heart_of_smartphones_intro.jpg")));
        assert_eq!(category.description, Some(String::from("An operating system (OS) is system software that manages computer hardware and software resources, and provides common services for computer programs.")));
    }
}
