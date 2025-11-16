use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Copy)]
pub enum CommentSortField {
    MostLikes,
    Latest,
}

#[derive(Debug, Clone, FromRow)]
pub struct Comment {
    pub id: i32,
    pub user_id: i32,
    pub quiz_id: i32,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct CommentDetail {
    pub comment: Comment,
    pub user_display_name: String,
    pub user_avatar_url: Option<String>,
    pub like_count: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct CreateCommentParams {
    pub user_id: i32,
    pub quiz_id: i32,
    pub content: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct UpdateCommentParams {
    pub id: i32,
    pub content: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct CommentQuery {
    pub quiz_id: i32,
    pub page: i32,
    pub limit: i32,
    pub sort_by: CommentSortField,
}
