use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, prelude::FromRow, Postgres};

use crate::models::{paginate::Paginate, question::Question};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct QuizResult {
    id: i32,
    quiz_title: String,
    score: f64,
    total_questions: i32,
    correct_answers: i32,
    submitted_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizResultQuery {
    user_id: i32,
    page: i32,
    size: i32,
}

impl Paginate<QuizResultQuery> for QuizResult {
    async fn page(
        query: &QuizResultQuery,
        connection: &PoolConnection<Postgres>,
    ) -> Result<super::paginate::Page<Self>, sqlx::Error> {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserAnswer {
    question: Question,
    explanation: Option<String>,
    chosen_option_ids: Vec<i32>,
    is_correct: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct QuizResultDetail {
    result: QuizResult,
    answers: Vec<UserAnswer>,
}

impl QuizResultDetail {
    pub async fn get_by_id(
        id: i32,
        connection: &PoolConnection<Postgres>,
    ) -> Result<QuizResultDetail, sqlx::Error> {
        todo!()
    }
}
