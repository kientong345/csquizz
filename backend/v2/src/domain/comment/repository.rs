use crate::domain::{
    comment::model::{
        Comment, CommentDetail, CommentQuery, CreateCommentParams, UpdateCommentParams,
    },
    error::RepositoryResult,
    page::Page,
};
use async_trait::async_trait;

#[async_trait]
pub trait CommentRepository: Send + Sync {
    /// Creates a new comment.
    async fn create(&self, params: &CreateCommentParams) -> RepositoryResult<Comment>;

    /// Finds a single comment by its ID.
    async fn find_by_id(&self, comment_id: i32) -> RepositoryResult<CommentDetail>;

    /// Lists comments for a quiz with details like author and like count.
    async fn find_all(&self, query: &CommentQuery) -> RepositoryResult<Page<CommentDetail>>;

    async fn update(&self, params: &UpdateCommentParams) -> RepositoryResult<Comment>;
    /// Deletes a comment by its ID.
    /// Note: Should add permission check in the service layer.
    async fn delete(&self, comment_id: i32) -> RepositoryResult<()>;
}
