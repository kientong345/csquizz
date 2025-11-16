use crate::domain::{
    category::model::{Category, CategoryQuery, CreateCategoryParams, UpdateCategoryParams},
    error::RepositoryResult,
    page::Page,
};
use async_trait::async_trait;

#[async_trait]
pub trait ICategoryRepository: Send + Sync {
    /// Creates a new category.
    async fn create_from(&self, param: &CreateCategoryParams) -> RepositoryResult<Category>;

    /// Finds a single category by its ID.
    async fn get_by(&self, category_id: i32) -> RepositoryResult<Category>;

    /// Lists all categories.
    async fn get_page_by(&self, query: &CategoryQuery) -> RepositoryResult<Page<Category>>;

    /// Updates an existing category.
    async fn update_by(
        &self,
        category_id: i32,
        params: &UpdateCategoryParams,
    ) -> RepositoryResult<Category>;

    /// Deletes a category by its ID.
    async fn delete_by(&self, category_id: i32) -> RepositoryResult<()>;
}
