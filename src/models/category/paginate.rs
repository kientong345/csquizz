use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

use crate::models::{
    category::QuizCategory,
    pagination::{Page, Paginate},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizCategoryQuery {
    pub page: i64,
    pub size: i64,
}

impl Paginate<QuizCategoryQuery> for QuizCategory {
    async fn page(
        query: &QuizCategoryQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, sqlx::Error> {
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

#[cfg(test)]
mod tests {
    use sqlx::{pool::PoolConnection, Postgres};

    use crate::{
        database::load_sample,
        models::{
            category::{paginate::QuizCategoryQuery, QuizCategory},
            pagination::Paginate,
        },
    };

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_category_page(mut conn: PoolConnection<Postgres>) {
        load_sample(&mut conn).await;

        let category_query = QuizCategoryQuery { page: 1, size: 10 };

        let category_page = QuizCategory::page(&category_query, &mut conn)
            .await
            .unwrap();

        assert_eq!(category_page.total_items, 3);
        assert_eq!(category_page.total_pages, 1);
        assert_eq!(&category_page.items[0].name, "Data Structures");
        assert_eq!(category_page.items[1].image_url, None);
        assert_eq!(
            category_page.items[2].description,
            Some(String::from(
                "Questions about network protocols, layers, and concepts."
            ))
        );
    }
}
