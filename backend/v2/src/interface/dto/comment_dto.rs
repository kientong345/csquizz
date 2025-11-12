use serde::{Deserialize, Serialize};
use crate::domain::models::comment::Comment;

// --- Query Structs ---

#[derive(Debug, Deserialize)]
pub struct ListCommentsQuery {
    #[serde(rename = "quizId")]
    pub quiz_id: i32,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}


// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateCommentDto {
    #[serde(rename = "quizId")]
    pub quiz_id: i32,
    pub content: String,
}


// --- Response DTOs ---

#[derive(Debug, Serialize)]
pub struct CommentDto {
    pub id: i32,
    #[serde(rename = "userId")]
    pub user_id: Option<i32>,
    // In a real app, you'd join to get user info
    #[serde(rename = "userDisplayName")]
    pub user_display_name: String, 
    pub content: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl CommentDto {
    pub fn from_domain(comment: Comment, user_display_name: String) -> Self {
        Self {
            id: comment.cmt_id,
            user_id: comment.cmt_user_id,
            user_display_name,
            content: comment.cmt_content,
            created_at: comment.cmt_created_at.map_or_else(String::new, |d| d.to_rfc3339()),
        }
    }
}
