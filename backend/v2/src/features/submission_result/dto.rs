use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

//================================================================================
// Request DTOs
//================================================================================

#[derive(Debug, Deserialize, ToSchema)]
pub struct AnswerRequestDto {
    pub question_id: i32,
    /// User's answer data, e.g., `{"option_index": 0}` or `{"entry": "text"}`
    pub data: Value,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SubmissionRequestDto {
    pub quiz_id: i32,
    pub answers: Vec<AnswerRequestDto>,
}

//================================================================================
// Response DTOs
//================================================================================

#[derive(Debug, Serialize, ToSchema)]
pub struct AnswerDetailDto {
    pub question_id: i32,
    pub user_answer: Value,
    pub correct_answer: Value,
    pub is_correct: bool,
    pub explanation: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubmissionDetailDto {
    pub id: i32,
    pub user_id: i32,
    pub quiz_id: i32,
    pub score: f32,
    pub submitted_at: String,
    pub details: Vec<AnswerDetailDto>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubmissionHistoryDto {
    pub submission_id: i32,
    pub quiz_id: i32,
    pub quiz_title: String,
    pub score: f32,
    pub submitted_at: String,
}
