use sqlx::PgConnection;

use crate::models::{category::Category, error::ModelError};

impl Category {
    pub async fn get_by_id(id: i32, connection: &mut PgConnection) -> Result<Category, ModelError> {
        Ok(sqlx::query_as!(
            Category,
            r#"SELECT cat_id AS id, cat_name AS name, cat_image_url AS image_url, cat_description AS description
            FROM categories
            WHERE cat_id = $1"#,
            id
        )
        .fetch_one(connection)
        .await?)
    }

    pub async fn list_all(connection: &mut PgConnection) -> Result<Vec<Category>, ModelError> {
        Ok(sqlx::query_as!(
            Category,
            r#"SELECT cat_id AS id, cat_name AS name, cat_image_url AS image_url, cat_description AS description
            FROM categories
            ORDER BY cat_id ASC"#
        )
        .fetch_all(connection)
        .await?)
    }

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
