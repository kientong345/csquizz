use async_trait::async_trait;
use crate::domain::models::submission::SubmissionResult;
use crate::interface::dto::submission_dto::ListSubmissionsQuery;
use super::error::RepositoryResult;

#[async_trait]
pub trait SubmissionRepository: Send + Sync {
    // In a real transaction, you would save the submission and its answers together.
    // This might be a more complex method in the service layer that uses multiple repository methods.
    async fn create(&self, user_id: i32, quiz_id: i32, score: f32) -> RepositoryResult<SubmissionResult>;

    async fn find_by_id(&self, submission_id: i32) -> RepositoryResult<Option<SubmissionResult>>;
    
    async fn list_by_user(&self, user_id: i32, query: &ListSubmissionsQuery) -> RepositoryResult<Vec<SubmissionResult>>;
}
