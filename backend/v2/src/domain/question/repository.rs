use crate::domain::{
    error::RepositoryResult,
    page::Page,
    question::model::{CreateQuestionParams, Question, UpdateQuestionParams},
};
use async_trait::async_trait;

#[async_trait]
pub trait QuestionRepository: Send + Sync {
    async fn create(&self, params: &CreateQuestionParams) -> RepositoryResult<Question>;

    async fn find_by_id(&self, question_id: i32) -> RepositoryResult<Question>;

    async fn find_all(&self, quiz_id: i32) -> RepositoryResult<Page<Question>>;

    async fn update(&self, params: &UpdateQuestionParams) -> RepositoryResult<Question>;

    async fn delete(&self, question_id: i32) -> RepositoryResult<()>;
}
