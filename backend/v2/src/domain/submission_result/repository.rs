use crate::domain::{
    error::RepositoryResult,
    page::Page,
    submission_result::model::{
        CreateSubmissionResult, SubmissionResult, SubmissionResultDetail, SubmissionResultMinimal,
        SubmissionResultQuery,
    },
};
use async_trait::async_trait;

#[async_trait]
pub trait SubmissionRepository: Send + Sync {
    async fn create(
        &self,
        params: &CreateSubmissionResult,
    ) -> RepositoryResult<SubmissionResult>;

    async fn find_by_id(&self, submission_id: i32) -> RepositoryResult<SubmissionResultDetail>;

    async fn find_all(
        &self,
        user_id: i32,
        query: &SubmissionResultQuery,
    ) -> RepositoryResult<Page<SubmissionResultMinimal>>;

    async fn delete(&self, submission_id: i32) -> RepositoryResult<()>;
}
