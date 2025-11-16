use crate::domain::{
    error::RepositoryResult,
    page::Page,
    question::model::{CreateQuestionParams, Question, UpdateQuestionParams},
};
use async_trait::async_trait;

#[async_trait]
pub trait IQuestionRepository: Send + Sync {
    async fn create_from(&self, params: &CreateQuestionParams) -> RepositoryResult<Question>;

    async fn get_by(&self, question_id: i32) -> RepositoryResult<Question>;

    async fn get_page_by(&self, quiz_id: i32) -> RepositoryResult<Page<Question>>;

    async fn update_by(&self, params: &UpdateQuestionParams) -> RepositoryResult<Question>;

    async fn delete_by(&self, question_id: i32) -> RepositoryResult<()>;
}
