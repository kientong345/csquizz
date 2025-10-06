use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, prelude::FromRow, Postgres};

use crate::models::paginate::Paginate;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct PossibleOption {
    id: i32,
    text: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Question {
    id: i32,
    question_type: String,
    question_text: String,
    possible_options: Vec<PossibleOption>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionQuery {}

impl Paginate<QuestionQuery> for Question {
    async fn page(
        query: &QuestionQuery,
        connection: &PoolConnection<Postgres>,
    ) -> Result<super::paginate::Page<Self>, sqlx::Error> {
        todo!()
    }
}
