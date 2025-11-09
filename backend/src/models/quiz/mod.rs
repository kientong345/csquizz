use serde::{Deserialize, Serialize};
use sqlx::{
    PgConnection,
    prelude::{FromRow, Type},
};

use crate::models::error::ModelError;

pub mod get;
pub mod paginate;
pub mod post;

#[derive(Debug, Type, Deserialize, Serialize, PartialEq, Eq)]
#[sqlx(type_name = "quiz_difficulty", rename_all = "kebab-case")]
pub enum QuizDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizMinimal {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub difficulty: Option<QuizDifficulty>,
}

#[derive(Debug, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct QuizMetadata {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub question_count: i64,
    pub difficulty: Option<QuizDifficulty>,
    pub created_by: Option<String>,
}

impl QuizMetadata {
    pub async fn count_by_creator_id(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(
            sqlx::query_scalar("SELECT COUNT(*) FROM quizzes WHERE created_by = $1")
                .bind(user_id)
                .fetch_one(connection)
                .await?,
        )
    }
}
