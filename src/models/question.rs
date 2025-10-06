use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, prelude::FromRow, Postgres};

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

impl Question {
    pub async fn get_by_quiz_id(
        id: i32,
        connection: &PoolConnection<Postgres>,
    ) -> Result<Vec<Question>, sqlx::Error> {
        todo!()
    }
}
