use serde::{Deserialize, Serialize};
use crate::domain::models::question::{Question, QuestionType};

// --- Query Structs ---

#[derive(Debug, Deserialize)]
pub struct ListQuestionsQuery {
    #[serde(rename = "quizId")]
    pub quiz_id: i32,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}


// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateQuestionDto {
    #[serde(rename = "quizId")]
    pub quiz_id: i32,
    #[serde(rename = "type")]
    pub question_type: QuestionType,
    pub content: String,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    // Key chứa đáp án, ví dụ: { "options": [...] } hoặc { "correctEntry": "..." }
    pub key: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuestionDto {
    #[serde(rename = "type")]
    pub question_type: Option<QuestionType>,
    pub content: Option<String>,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    pub key: Option<serde_json::Value>,
}


// --- Response DTOs ---

#[derive(Debug, Serialize)]
pub struct QuestionDto {
    pub id: i32,
    #[serde(rename = "type")]
    pub question_type: QuestionType,
    pub content: String,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    #[serde(rename = "quizId")]
    pub quiz_id: Option<i32>,
    // Key sẽ được trả về cho admin hoặc trong màn hình kết quả chi tiết
    pub key: serde_json::Value,
}

impl From<Question> for QuestionDto {
    fn from(q: Question) -> Self {
        Self {
            id: q.qs_id,
            question_type: q.qs_type,
            content: q.qs_content,
            image_url: q.qs_image_url,
            quiz_id: q.qs_quiz_id,
            key: q.qs_key,
        }
    }
}
