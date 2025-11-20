use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    domain::{
        error::RepositoryResult,
        page::Page,
        question::{
            model::{CreateQuestionParams, Question, UpdateQuestionParams},
            repository::QuestionRepository,
        },
    },
    infrastructure::database::postgres_context::DatabasePool,
};

pub struct SqlxQuestionRepository {
    pool: Arc<DatabasePool>,
}

impl SqlxQuestionRepository {
    pub fn init(pool: Arc<DatabasePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QuestionRepository for SqlxQuestionRepository {
    async fn create(&self, params: &CreateQuestionParams) -> RepositoryResult<Question> {
        todo!()
    }

    async fn find_by_id(&self, question_id: i32) -> RepositoryResult<Question> {
        todo!()
    }

    async fn find_all(&self, quiz_id: i32) -> RepositoryResult<Page<Question>> {
        todo!()
    }

    async fn update(&self, params: &UpdateQuestionParams) -> RepositoryResult<Question> {
        todo!()
    }

    async fn delete(&self, question_id: i32) -> RepositoryResult<()> {
        todo!()
    }
}
