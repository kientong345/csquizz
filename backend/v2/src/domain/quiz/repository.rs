use crate::domain::{
    error::RepositoryResult,
    page::Page,
    quiz::model::{CreateQuizParams, Quiz, QuizDetail, QuizMinimal, QuizQuery, UpdateQuizParams},
};
use async_trait::async_trait;

#[async_trait]
pub trait QuizRepository: Send + Sync {
    /// Creates a new quiz.
    async fn create(&self, params: &CreateQuizParams) -> RepositoryResult<Quiz>;

    /// Finds the detailed information for a single quiz by its ID.
    async fn find_by_id(&self, quiz_id: i32) -> RepositoryResult<QuizDetail>;

    /// Lists quizzes with minimal information, including counts.
    async fn find_all(&self, query: &QuizQuery) -> RepositoryResult<Page<QuizMinimal>>;

    /// Updates an existing quiz.
    async fn update(&self, quiz_id: i32, params: &UpdateQuizParams) -> RepositoryResult<Quiz>;

    /// Deletes a quiz by its ID.
    async fn delete(&self, quiz_id: i32) -> RepositoryResult<()>;
}
