use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection};

use crate::models::question::Question;

pub mod get;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct QuizResult {
    pub id: i32,
    pub quiz_title: String,
    pub score: f64,
    pub total_questions: i32,
    pub correct_answers: i32,
    // pub submitted_at: DateTime<Utc>,
}

impl QuizResult {
    pub async fn count_by_user_id(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, sqlx::Error> {
        Ok(
            sqlx::query_scalar("SELECT COUNT(*) FROM results WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(connection)
                .await?,
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizResultQuery {
    pub user_id: i32,
    pub page: i64,
    pub size: i64,
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
