use async_trait::async_trait;

use crate::domain::error::RepositoryResult;

#[async_trait]
pub trait ILikeRepository: Send + Sync {
    async fn create_quiz_like(&self, user_id: i32, quiz_id: i32) -> RepositoryResult<()>;

    async fn delete_quiz_like(&self, quiz_like_id: i32) -> RepositoryResult<()>;

    async fn create_comment_like(&self, user_id: i32, comment_id: i32) -> RepositoryResult<()>;

    async fn delete_comment_like(&self, comment_like_id: i32) -> RepositoryResult<()>;
}
