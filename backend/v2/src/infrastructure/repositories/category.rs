use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    domain::{
        category::{
            model::{Category, CategoryQuery, CreateCategoryParams, UpdateCategoryParams},
            repository::CategoryRepository,
        },
        error::RepositoryResult,
        page::Page,
    },
    infrastructure::database::postgres_context::DatabasePool,
};

pub struct SqlxCategoryRepository {
    pool: Arc<DatabasePool>,
}

impl SqlxCategoryRepository {
    pub fn init(pool: Arc<DatabasePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CategoryRepository for SqlxCategoryRepository {
    async fn create(&self, params: &CreateCategoryParams) -> RepositoryResult<Category> {
        // let category = sqlx::query_as!(
        //     Category,
        //     r#"
        //     INSERT INTO categories (cat_name, cat_image_url, cat_description)
        //     VALUES ($1, $2, $3)
        //     RETURNING cat_id as id, cat_name as name, cat_image_url as image_url, cat_description as description;
        //     "#,
        //     params.name,
        //     params.image_url,
        //     params.description
        // )
        // .fetch_one(&self.pool)
        // .await?;

        // Ok(category)
        todo!()
    }

    async fn find_by_id(&self, category_id: i32) -> RepositoryResult<Category> {
        let mut connection = self.pool.get_connection().await?;
        let category = sqlx::query_as!(
            Category,
            r#"
            SELECT cat_id as id, cat_name as name, cat_image_url as image_url, cat_description as description
            FROM categories
            WHERE cat_id = $1
            "#,
            category_id
        )
        .fetch_one(&mut *connection)
        .await?;

        Ok(category)
    }

    async fn find_all(&self, query: &CategoryQuery) -> RepositoryResult<Page<Category>> {
        // let categories = sqlx::query_as!(
        //     Category,
        //     r#"
        //     SELECT cat_id as id, cat_name as name, cat_image_url as image_url, cat_description as description
        //     FROM categories
        //     ORDER BY cat_name;
        //     "#
        // )
        // .fetch_all(&self.pool)
        // .await?;

        // Ok(categories)
        todo!()
    }

    async fn update(
        &self,
        category_id: i32,
        params: &UpdateCategoryParams,
    ) -> RepositoryResult<Category> {
        // let category = sqlx::query_as!(
        //     Category,
        //     r#"
        //     UPDATE categories
        //     SET
        //         cat_name = COALESCE($1, cat_name),
        //         cat_image_url = COALESCE($2, cat_image_url),
        //         cat_description = COALESCE($3, cat_description)
        //     WHERE cat_id = $4
        //     RETURNING cat_id as id, cat_name as name, cat_image_url as image_url, cat_description as description;
        //     "#,
        //     params.name,
        //     params.image_url,
        //     params.description,
        //     category_id
        // )
        // .fetch_one(&self.pool)
        // .await?;

        // Ok(category)
        todo!()
    }

    async fn delete(&self, category_id: i32) -> RepositoryResult<()> {
        // sqlx::query!("DELETE FROM categories WHERE cat_id = $1", category_id)
        //     .execute(&self.pool)
        //     .await?;
        // Ok(())
        todo!()
    }
}
