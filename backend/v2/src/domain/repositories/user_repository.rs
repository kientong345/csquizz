use async_trait::async_trait;
use crate::domain::models::user::{User, UserRole};
use super::error::RepositoryResult;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, display_name: &str, email: &str, password_hash: &str) -> RepositoryResult<User>;
    
    async fn find_by_id(&self, user_id: i32) -> RepositoryResult<Option<User>>;

    async fn find_by_email(&self, email: &str) -> RepositoryResult<Option<User>>;

    async fn update_role(&self, user_id: i32, role: UserRole) -> RepositoryResult<()>;
    
    // You might add list(&self, query: ListUsersQuery) -> RepositoryResult<Vec<User>> later
}
