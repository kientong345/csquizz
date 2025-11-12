use async_trait::async_trait;
use crate::domain::models::category::Category;
use crate::interface::dto::category_dto::{CreateCategoryDto, UpdateCategoryDto};
use super::error::RepositoryResult;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn create(&self, dto: &CreateCategoryDto) -> RepositoryResult<Category>;

    async fn find_by_id(&self, category_id: i32) -> RepositoryResult<Option<Category>>;
    
    async fn list(&self) -> RepositoryResult<Vec<Category>>;

    async fn update(&self, category_id: i32, dto: &UpdateCategoryDto) -> RepositoryResult<Category>;

    async fn delete(&self, category_id: i32) -> RepositoryResult<()>;
}
