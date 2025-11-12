use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Struct đại diện cho một bình luận về một bài quiz.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Comment {
    #[serde(rename = "id")]
    pub cmt_id: i32,

    #[serde(rename = "userId")]
    pub cmt_user_id: Option<i32>,

    #[serde(rename = "quizId")]
    pub cmt_quiz_id: Option<i32>,

    #[serde(rename = "content")]
    pub cmt_content: String,

    #[serde(rename = "createdAt")]
    pub cmt_created_at: Option<DateTime<Utc>>,
}
