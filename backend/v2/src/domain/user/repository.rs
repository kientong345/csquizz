use crate::domain::{
    error::RepositoryResult,
    page::Page,
    user::model::{CreateUserParams, UpdateUserParams, User, UserDetail, UserMinimal, UserQuery},
};
use async_trait::async_trait;

#[async_trait]
pub trait IUserRepository: Send + Sync {
    async fn create_from(&self, params: &CreateUserParams) -> RepositoryResult<User>;

    async fn get_by(&self, user_id: i32) -> RepositoryResult<UserDetail>;

    async fn update_by(&self, params: &UpdateUserParams) -> RepositoryResult<()>;

    async fn get_page_by(&self, query: &UserQuery) -> RepositoryResult<Page<UserMinimal>>;

    async fn delete_by(&self, user_id: i32) -> RepositoryResult<()>;
}
