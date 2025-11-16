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
pub trait ISubmissionRepository: Send + Sync {
    async fn create_from(
        &self,
        params: &CreateSubmissionResult,
    ) -> RepositoryResult<SubmissionResult>;

    async fn get_by(&self, submission_id: i32) -> RepositoryResult<SubmissionResultDetail>;

    async fn get_page_by(
        &self,
        user_id: i32,
        query: &SubmissionResultQuery,
    ) -> RepositoryResult<Page<SubmissionResultMinimal>>;

    async fn delete_by(&self, submission_id: i32) -> RepositoryResult<()>;
}
