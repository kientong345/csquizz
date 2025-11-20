use crate::domain::{
    category::model::{Category, CategoryQuery, CreateCategoryParams, UpdateCategoryParams},
    error::RepositoryResult,
    page::Page,
};
use async_trait::async_trait;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    /// Creates a new category.
    async fn create(&self, param: &CreateCategoryParams) -> RepositoryResult<Category>;

    /// Finds a single category by its ID.
    async fn find_by_id(&self, category_id: i32) -> RepositoryResult<Category>;

    /// Lists all categories.
    async fn find_all(&self, query: &CategoryQuery) -> RepositoryResult<Page<Category>>;

    /// Updates an existing category.
    async fn update(
        &self,
        category_id: i32,
        params: &UpdateCategoryParams,
    ) -> RepositoryResult<Category>;

    /// Deletes a category by its ID.
    async fn delete(&self, category_id: i32) -> RepositoryResult<()>;
}
