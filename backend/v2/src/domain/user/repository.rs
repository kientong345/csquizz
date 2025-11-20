use crate::domain::{
    error::RepositoryResult,
    page::Page,
    user::model::{CreateUserParams, UpdateUserParams, User, UserDetail, UserMinimal, UserQuery},
};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, params: &CreateUserParams) -> RepositoryResult<User>;

    async fn find_by_id(&self, user_id: i32) -> RepositoryResult<UserDetail>;

    async fn update(&self, params: &UpdateUserParams) -> RepositoryResult<()>;

    async fn find_all(&self, query: &UserQuery) -> RepositoryResult<Page<UserMinimal>>;

    async fn delete(&self, user_id: i32) -> RepositoryResult<()>;
}
