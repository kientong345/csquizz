use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection};

use crate::models::question::QuestionForm;

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

    pub async fn get_quiz_id_from(
        result_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i32, sqlx::Error> {
        Ok(
            sqlx::query!(r#"SELECT quiz_id FROM results WHERE id = $1"#, result_id)
                .fetch_one(connection)
                .await?
                .quiz_id
                .unwrap_or(-1),
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
    pub question_form: QuestionForm,
    pub question_text: String,
    pub question_image_url: Option<String>,
    pub options_text: Vec<String>,
    pub explanation: Option<String>,

    pub chosen_options_index: Vec<i32>,
    pub entried_text: Option<String>,
    pub is_correct: Option<bool>,
}

impl UserAnswer {
    pub async fn count_by_result_id(
        result_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, sqlx::Error> {
        Ok(
            sqlx::query_scalar("SELECT COUNT(*) FROM user_answers WHERE result_id = $1")
                .bind(result_id)
                .fetch_one(connection)
                .await?,
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserAnswerQuery {
    pub result_id: i32,
    pub page: i64,
    pub size: i64,
}
