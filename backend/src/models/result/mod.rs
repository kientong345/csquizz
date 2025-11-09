use crate::utils::{deserialize_snake_case, serializeCamelCase};
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, prelude::FromRow};

use crate::models::{
    error::ModelError,
    question::QuestionWithKey,
    quiz::{QuizDifficulty, QuizMinimal},
};

pub mod get;
pub mod paginate;
pub mod post;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserChoice {
    #[serde(
        serialize_with = "serializeCamelCase",
        deserialize_with = "deserialize_snake_case"
    )]
    pub option_index: i32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserEntry {
    #[serde(
        serialize_with = "serializeCamelCase",
        deserialize_with = "deserialize_snake_case"
    )]
    pub text_entried: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum UserAnswer {
    SingleChoiceAnswer(UserChoice),
    MultipleChoiceAnswer(Vec<UserChoice>),
    TextEntryAnswer(UserEntry),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionResult {
    pub question_with_key: QuestionWithKey,
    pub user_answer: UserAnswer,
}

impl QuestionResult {
    pub async fn count_by_result_id(
        result_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM user_answers WHERE result_id = $1"#,
            result_id
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0))
    }
}

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QuizResultSummary {
    pub id: i32,
    pub user_id: i32,
    pub quiz: QuizMinimal,
    pub score: f64,
    pub total_questions: i32,
    pub correct_answers: i32,
    // pub submitted_at: DateTime<Utc>,
}

impl QuizResultSummary {
    pub async fn count_by_user_id(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM results WHERE user_id = $1"#,
            user_id
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0))
    }

    pub async fn count_distinct_by_user_id(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(DISTINCT user_id) FROM results WHERE user_id = $1"#,
            user_id
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0))
    }

    pub async fn get_quiz_id_from(
        result_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i32, ModelError> {
        Ok(
            sqlx::query!(r#"SELECT quiz_id FROM results WHERE id = $1"#, result_id)
                .fetch_one(connection)
                .await?
                .quiz_id
                .unwrap_or(-1),
        )
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizResult {
    pub summary: QuizResultSummary,
    pub result: Vec<QuestionResult>,
}

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct TmpQuizResultSummary {
    pub quiz: QuizMinimal,
    pub score: f64,
    pub total_questions: i32,
    pub correct_answers: i32,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TmpQuizResult {
    pub summary: TmpQuizResultSummary,
    pub result: Vec<QuestionResult>,
}

#[derive(Debug, FromRow)]
struct FetchedQuizSummary {
    id: i32,
    user_id: i32,
    quiz_id: i32,
    quiz_title: String,
    quiz_category: String,
    quiz_difficulty: Option<QuizDifficulty>,
    score: f64,
    total_questions: i32,
    correct_answers: i32,
    // submitted_at: DateTime<Utc>,
}
