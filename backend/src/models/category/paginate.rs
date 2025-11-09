use serde::Deserialize;
use sqlx::PgConnection;

use crate::models::{
    category::Category,
    error::ModelError,
    pagination::{Page, Paginate},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CategoryQuery {
    pub name_pattern: Option<String>,
    pub page: i64,
    pub size: i64,
}

impl Paginate<CategoryQuery> for Category {
    async fn page(
        query: &CategoryQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let name_pattern = format!("%{}%", query.name_pattern.clone().unwrap_or("".to_string()));
        let offset = (query.page.saturating_sub(1)) * query.size;

        let items = sqlx::query_as!(
            Category,
            r#"SELECT id, name, image_url, description FROM categories
            WHERE name LIKE $1 LIMIT $2 OFFSET $3"#,
            name_pattern,
            query.size,
            offset
        )
        .fetch_all(&mut *connection)
        .await?;

        let total_items = sqlx::query_scalar!(
            r#"SELECT COUNT(id) FROM categories WHERE name LIKE $1"#,
            name_pattern,
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0);

        Ok(Page::build_from(items, total_items, query.size))
    }
}

#[cfg(feature = "local")]
#[cfg(test)]
mod tests {
    use sqlx::{Postgres, pool::PoolConnection};

    use crate::{
        database::load_sample,
        models::{
            category::{Category, paginate::CategoryQuery},
            pagination::Paginate,
        },
    };

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_category_page(mut conn: PoolConnection<Postgres>) {
        load_sample(&mut conn).await;

        let category_query = CategoryQuery { page: 1, size: 10 };

        let category_page = Category::page(&category_query, &mut conn).await.unwrap();

        assert_eq!(category_page.total_items, 3);
        assert_eq!(category_page.total_pages, 1);
        assert_eq!(category_page.items[0].name, "Data Structures".to_string());
        assert_eq!(category_page.items[1].image_url, None);
        assert_eq!(
            category_page.items[2].description,
            Some(String::from(
                "Questions about network protocols, layers, and concepts."
            ))
        );
    }
}
