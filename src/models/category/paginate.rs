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
