use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Struct đại diện cho một lượt "like" của người dùng cho một bài quiz.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct QuizLike {
    #[serde(rename = "userId")]
    pub qzlk_user_id: Option<i32>,

    #[serde(rename = "quizId")]
    pub qzlk_quiz_id: Option<i32>,
}

/// Struct đại diện cho một lượt "like" của người dùng cho một bình luận.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CommentLike {
    #[serde(rename = "userId")]
    pub cmtlk_user_id: Option<i32>,

    #[serde(rename = "commentId")]
    pub cmtlk_comment_id: Option<i32>,
}
