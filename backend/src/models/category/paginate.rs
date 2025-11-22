use sqlx::PgConnection;

use crate::models::{
    category::{Category, CategoryPaginateParams},
    error::ModelError,
    pagination::{Page, Paginate},
};

impl Paginate<CategoryPaginateParams> for Category {
    async fn page(
        params: &CategoryPaginateParams,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let name_pattern = format!(
            "%{}%",
            params.name_pattern.clone().unwrap_or("".to_string())
        );
        let offset = (params.page.saturating_sub(1)) * params.page_size;

        let items = sqlx::query_as!(
            Category,
            r#"SELECT cat_id AS id, cat_name AS name, cat_image_url AS image_url, cat_description AS description
            FROM categories
            WHERE cat_name LIKE $1 LIMIT $2 OFFSET $3"#,
            name_pattern,
            params.page_size as i64,
            offset as i64
        )
        .fetch_all(&mut *connection)
        .await?;

        let total_items = sqlx::query_scalar!(
            r#"SELECT COUNT(cat_id) FROM categories WHERE cat_name LIKE $1"#,
            name_pattern,
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0);

        Ok(Page::build_from(items, total_items, params.page_size))
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
