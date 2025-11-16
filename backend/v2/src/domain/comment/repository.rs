use crate::domain::{
    comment::model::{
        Comment, CommentDetail, CommentQuery, CreateCommentParams, UpdateCommentParams,
    },
    error::RepositoryResult,
    page::Page,
};
use async_trait::async_trait;

#[async_trait]
pub trait ICommentRepository: Send + Sync {
    /// Creates a new comment.
    async fn create_from(&self, params: &CreateCommentParams) -> RepositoryResult<Comment>;

    /// Finds a single comment by its ID.
    async fn get_by(&self, comment_id: i32) -> RepositoryResult<CommentDetail>;

    /// Lists comments for a quiz with details like author and like count.
    async fn get_page_by(&self, query: &CommentQuery) -> RepositoryResult<Page<CommentDetail>>;

    async fn update_by(&self, params: &UpdateCommentParams) -> RepositoryResult<Comment>;
    /// Deletes a comment by its ID.
    /// Note: Should add permission check in the service layer.
    async fn delete_by(&self, comment_id: i32) -> RepositoryResult<()>;
}
