use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

/// Enum đại diện cho các loại câu hỏi.
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "question_type", rename_all = "snake_case")]
pub enum QuestionType {
    SingleChoice,
    MultipleChoice,
    TextEntry,
}

// --- Structs for Question Keys (qs_key) ---

/// Một lựa chọn trong câu hỏi trắc nghiệm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyOption {
    pub content: String,
    #[serde(rename = "imageUrl", skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(rename = "isCorrect")]
    pub is_correct: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

/// Key cho câu hỏi trắc nghiệm (một hoặc nhiều lựa chọn).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionKeys {
    pub keys: Vec<KeyOption>,
}

/// Key cho câu hỏi điền từ.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextKey {
    #[serde(rename = "correctEntry")]
    pub correct_entry: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

// --- Main Question Struct ---

/// Struct đại diện cho một câu hỏi trong quiz.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Question {
    #[serde(rename = "id")]
    pub qs_id: i32,

    #[serde(rename = "type")]
    pub qs_type: QuestionType,

    #[serde(rename = "content")]
    pub qs_content: String,

    #[serde(rename = "imageUrl")]
    pub qs_image_url: Option<String>,

    #[serde(rename = "key")]
    #[sqlx(json)]
    pub qs_key: Value, // Dùng serde_json::Value để linh hoạt, parse sau ở application layer

    #[serde(rename = "quizId")]
    pub qs_quiz_id: Option<i32>,

    #[serde(rename = "createdAt")]
    pub qs_created_at: Option<DateTime<Utc>>,
}
