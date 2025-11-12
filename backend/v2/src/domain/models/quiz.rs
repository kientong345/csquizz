use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Enum đại diện cho độ khó của một bài quiz.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "quiz_difficulty", rename_all = "lowercase")]
pub enum QuizDifficulty {
    Easy,
    Medium,
    Hard,
}

/// Struct đại diện cho một bài quiz.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Quiz {
    #[serde(rename = "id")]
    pub qz_id: i32,

    #[serde(rename = "title")]
    pub qz_title: String,

    #[serde(rename = "description")]
    pub qz_description: Option<String>,

    #[serde(rename = "difficulty")]
    pub qz_difficulty: QuizDifficulty,

    #[serde(rename = "categoryId")]
    pub qz_category_id: Option<i32>,

    #[serde(rename = "creatorId")]
    pub qz_creator_id: Option<i32>,

    #[serde(rename = "createdAt")]
    pub qz_created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt")]
    pub qz_updated_at: Option<DateTime<Utc>>,
}
