use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

/// Enum đại diện cho các loại câu hỏi.
#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "question_type", rename_all = "snake_case")]
pub enum QuestionType {
    SingleChoice,
    MultipleChoice,
    TextEntry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyOption {
    pub content: String,
    pub image_url: Option<String>,
    pub is_correct: bool,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionKeys {
    pub keys: Vec<KeyOption>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextKey {
    pub correct_entry: String,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Question {
    pub id: i32,
    pub r#type: QuestionType,
    pub content: String,
    pub image_url: Option<String>,
    #[sqlx(json)]
    pub key: Value,
    pub quiz_id: i32,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct CreateQuestionParams {
    pub quiz_id: i32,
    pub r#type: QuestionType,
    pub content: String,
    pub image_url: Option<String>,
    pub key: Value,
}

#[derive(Debug, Clone, FromRow)]
pub struct UpdateQuestionParams {
    pub id: i32,
    pub r#type: Option<QuestionType>,
    pub content: Option<String>,
    pub image_url: Option<String>,
    pub key: Option<Value>,
}
