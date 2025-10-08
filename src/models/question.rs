use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, prelude::FromRow, PgConnection, Postgres};

use crate::models::paginate::Paginate;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct AnswerOption {
    id: i32,
    text: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Question {
    id: i32,
    form: String,
    text: String,
    options: Vec<AnswerOption>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionQuery {
    quiz_id: i32,
    page: i32,
    size: i32,
}

impl Paginate<QuestionQuery> for Question {
    async fn page(
        query: &QuestionQuery,
        connection: &mut PgConnection,
    ) -> Result<super::paginate::Page<Self>, sqlx::Error> {
        todo!()
    }
}
