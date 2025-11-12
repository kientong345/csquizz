use serde::{Deserialize, Serialize};

// --- Query Structs ---

#[derive(Debug, Deserialize)]
pub struct ListSubmissionsQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}


// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct SubmitQuizDto {
    #[serde(rename = "quizId")]
    pub quiz_id: i32,
    pub answers: Vec<QuizSubmissionAnswerDto>,
}

#[derive(Debug, Deserialize)]
pub struct QuizSubmissionAnswerDto {
    #[serde(rename = "questionId")]
    pub question_id: i32,
    // Dữ liệu trả lời của người dùng, ví dụ: { "optionIndex": 0 } hoặc { "entry": "text" }
    pub data: serde_json::Value,
}


// --- Response DTOs ---

#[derive(Debug, Serialize)]
pub struct QuizResultDto {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: i32,
    #[serde(rename = "quizId")]
    pub quiz_id: i32,
    pub score: f32,
    #[serde(rename = "submittedAt")]
    pub submitted_at: String,
    pub details: Vec<AnswerResultDetailDto>,
}

#[derive(Debug, Serialize)]
pub struct AnswerResultDetailDto {
    #[serde(rename = "questionId")]
    pub question_id: i32,
    #[serde(rename = "userAnswer")]
    pub user_answer: serde_json::Value,
    #[serde(rename = "correctAnswer")]
    pub correct_answer: serde_json::Value,
    #[serde(rename = "isCorrect")]
    pub is_correct: bool,
    pub explanation: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SubmissionHistoryItemDto {
    #[serde(rename = "submissionId")]
    pub submission_id: i32,
    #[serde(rename = "quizId")]
    pub quiz_id: i32,
    #[serde(rename = "quizTitle")]
    pub quiz_title: String,
    pub score: f32,
    #[serde(rename = "submittedAt")]
    pub submitted_at: String,
}
