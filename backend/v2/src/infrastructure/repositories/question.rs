use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    domain::{
        error::RepositoryResult,
        page::Page,
        question::{
            model::{CreateQuestionParams, Question, UpdateQuestionParams},
            repository::IQuestionRepository,
        },
    },
    infrastructure::database::postgres_context::DatabasePool,
};

pub struct QuestionRepository {
    pool: Arc<DatabasePool>,
}

impl QuestionRepository {
    pub fn init(pool: Arc<DatabasePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IQuestionRepository for QuestionRepository {
    async fn create_from(&self, params: &CreateQuestionParams) -> RepositoryResult<Question> {
        todo!()
    }

    async fn get_by(&self, question_id: i32) -> RepositoryResult<Question> {
        todo!()
    }

    async fn get_page_by(&self, quiz_id: i32) -> RepositoryResult<Page<Question>> {
        todo!()
    }

    async fn update_by(&self, params: &UpdateQuestionParams) -> RepositoryResult<Question> {
        todo!()
    }

    async fn delete_by(&self, question_id: i32) -> RepositoryResult<()> {
        todo!()
    }
}
