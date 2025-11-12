use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

// --- Structs for Answer Data (ans_data) ---

/// Câu trả lời của người dùng cho câu hỏi trắc nghiệm nhiều lựa chọn.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserChoices {
    pub choices: Vec<UserChoice>,
}

/// Câu trả lời của người dùng cho câu hỏi trắc nghiệm một lựa chọn.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserChoice {
    #[serde(rename = "optionIndex")]
    pub option_index: i32,
}

/// Câu trả lời của người dùng cho câu hỏi điền từ.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntry {
    pub entry: String,
}

// --- Main Submission and Answer Structs ---

/// Struct đại diện cho một câu trả lời của người dùng.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Answer {
    #[serde(rename = "id")]
    pub ans_id: i32,

    #[serde(rename = "resultId")]
    pub ans_result_id: Option<i32>,

    #[serde(rename = "questionId")]
    pub ans_question_id: Option<i32>,

    #[serde(rename = "data")]
    #[sqlx(json)]
    pub ans_data: Value, // Dùng serde_json::Value, parse ở application layer
}

/// Struct đại diện cho kết quả của một lần nộp bài.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct SubmissionResult {
    #[serde(rename = "id")]
    pub sub_id: i32,

    #[serde(rename = "userId")]
    pub sub_user_id: Option<i32>,

    #[serde(rename = "quizId")]
    pub sub_quiz_id: Option<i32>,

    #[serde(rename = "score")]
    pub sub_score: f32,

    #[serde(rename = "submittedAt")]
    pub sub_submitted_at: Option<DateTime<Utc>>,
}
