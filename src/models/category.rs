use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection};

use crate::models::paginate::{Page, Paginate};

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

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizCategoryQuery {
    pub page: i64,
    pub size: i64,
}

impl Paginate<QuizCategoryQuery> for QuizCategory {
    async fn page(
        query: &QuizCategoryQuery,
        connection: &mut PgConnection,
    ) -> Result<super::paginate::Page<Self>, sqlx::Error> {
        let total_items = QuizCategory::count(connection).await?;
        let offset = (query.page.saturating_sub(1)) * query.size;

        let items = sqlx::query_as!(
            QuizCategory,
            r#"SELECT id, name, image_url, description FROM categories LIMIT $1 OFFSET $2"#,
            query.size,
            offset
        )
        .fetch_all(connection)
        .await?;

        Ok(Page::create_from(items, total_items, query.size))
    }
}
