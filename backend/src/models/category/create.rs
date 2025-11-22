use sqlx::PgConnection;

use crate::models::{
    category::{Category, CategoryCreateParams},
    error::ModelError,
};

impl Category {
    pub async fn create_from(
        params: CategoryCreateParams,
        connection: &mut PgConnection,
    ) -> Result<Category, ModelError> {
        Ok(sqlx::query_as!(
            Category,
            r#"INSERT INTO categories (cat_name, cat_image_url, cat_description)
            VALUES ($1, $2, $3)
            RETURNING cat_id AS id, cat_name AS name, cat_image_url AS image_url, cat_description AS description"#,
            params.name,
            params.image_url,
            params.description
        )
        .fetch_one(connection)
        .await?)
    }
}

#[cfg(feature = "local")]
#[cfg(test)]
mod tests {
    use sqlx::{Postgres, pool::PoolConnection};

    use crate::models::category::{Category, create::PostCategory};

    #[sqlx::test(migrations = "./migrations")]
    async fn test_create_new_category(mut conn: PoolConnection<Postgres>) {
        let data = PostCategory {
            name: String::from("Operating System"),
            image_url: Some(String::from(
                "https://www.telecomreviewafrica.com/wp-content/uploads/2019/12/Operating_systemsThe_heart_of_smartphones_intro.jpg",
            )),
            description: Some(String::from(
                "An operating system (OS) is system software that manages computer hardware and software resources, and provides common services for computer programs.",
            )),
        };

        let category = Category::create_from(data, &mut conn).await.unwrap();

        assert_eq!(category.name, "Operating System".to_string());
        assert_eq!(
            category.image_url,
            Some(String::from(
                "https://www.telecomreviewafrica.com/wp-content/uploads/2019/12/Operating_systemsThe_heart_of_smartphones_intro.jpg"
            ))
        );
        assert_eq!(
            category.description,
            Some(String::from(
                "An operating system (OS) is system software that manages computer hardware and software resources, and provides common services for computer programs."
            ))
        );
    }
}
