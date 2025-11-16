use serde::Deserialize;

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct QuizLikeDto {
    #[serde(rename = "quizId")]
    pub quiz_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct CommentLikeDto {
    #[serde(rename = "commentId")]
    pub comment_id: i32,
}
