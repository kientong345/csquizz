use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection};

use crate::models::{paginate::Paginate, question::Question};

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct QuizResult {
    pub id: i32,
    pub quiz_title: String,
    pub score: f64,
    pub total_questions: i32,
    pub correct_answers: i32,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizResultQuery {
    pub user_id: i32,
    pub page: i32,
    pub size: i32,
}

impl Paginate<QuizResultQuery> for QuizResult {
    async fn page(
        query: &QuizResultQuery,
        connection: &mut PgConnection,
    ) -> Result<super::paginate::Page<Self>, sqlx::Error> {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserAnswer {
    pub question: Question,
    pub explanation: Option<String>,
    pub chosen_option_ids: Vec<i32>,
    pub text_answer: Option<String>,
    pub is_correct: bool,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct QuizResultDetail {
    pub result: QuizResult,
    pub answers: Vec<UserAnswer>,
}

impl QuizResultDetail {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuizResultDetail, sqlx::Error> {
        todo!()
    }
}
