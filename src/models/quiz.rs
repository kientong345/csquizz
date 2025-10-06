use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, prelude::FromRow, Postgres};

use crate::models::{paginate::Paginate, question::Question};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Quiz {
    id: i32,
    title: String,
    description: Option<String>,
    category: String,
    difficulty: Option<String>,
    created_by: Option<String>,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizQuery {}

impl Paginate<QuizQuery> for Quiz {
    async fn page(
        query: &QuizQuery,
        connection: &PoolConnection<Postgres>,
    ) -> Result<super::paginate::Page<Self>, sqlx::Error> {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct QuizDetail {
    quiz: Quiz,
    questions: Vec<Question>,
}

impl QuizDetail {
    pub async fn get_by_id(
        id: i32,
        connection: &PoolConnection<Postgres>,
    ) -> Result<QuizDetail, sqlx::Error> {
        // let quiz = Quiz::get_by_id(id, connection).await?;
        // let questions = Question::get_by_quiz_id(id, connection).await?;
        // Ok(QuizDetail { quiz, questions })
        todo!()
    }
}
