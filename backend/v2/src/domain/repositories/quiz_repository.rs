use async_trait::async_trait;
use crate::domain::models::quiz::Quiz;
use crate::interface::dto::quiz_dto::{CreateQuizDto, UpdateQuizDto, ListQuizzesQuery};
use super::error::RepositoryResult;

#[async_trait]
pub trait QuizRepository: Send + Sync {
    async fn create(&self, dto: &CreateQuizDto, creator_id: i32) -> RepositoryResult<Quiz>;

    async fn find_by_id(&self, quiz_id: i32) -> RepositoryResult<Option<Quiz>>;
    
    async fn list(&self, query: &ListQuizzesQuery) -> RepositoryResult<Vec<Quiz>>;

    async fn update(&self, quiz_id: i32, dto: &UpdateQuizDto) -> RepositoryResult<Quiz>;

    async fn delete(&self, quiz_id: i32) -> RepositoryResult<()>;
}
