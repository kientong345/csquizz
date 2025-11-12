use async_trait::async_trait;
use crate::domain::models::question::Question;
use crate::interface::dto::question_dto::{CreateQuestionDto, UpdateQuestionDto, ListQuestionsQuery};
use super::error::RepositoryResult;

#[async_trait]
pub trait QuestionRepository: Send + Sync {
    async fn create(&self, dto: &CreateQuestionDto) -> RepositoryResult<Question>;

    async fn find_by_id(&self, question_id: i32) -> RepositoryResult<Option<Question>>;
    
    async fn list(&self, query: &ListQuestionsQuery) -> RepositoryResult<Vec<Question>>;

    async fn update(&self, question_id: i32, dto: &UpdateQuestionDto) -> RepositoryResult<Question>;

    async fn delete(&self, question_id: i32) -> RepositoryResult<()>;
}
