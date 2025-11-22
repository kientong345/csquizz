use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod create;
pub mod delete;
pub mod get;
pub mod paginate;
pub mod update;

#[derive(Debug, Clone, FromRow)]
pub struct DatabaseComment {
    pub id: i32,
    pub user_id: i32,
    pub quiz_id: i32,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentDetail {
    pub id: i32,
    pub user_id: i32,
    pub quiz_id: i32,
    pub content: String,
    pub created_at: Option<String>,
    pub user_display_name: String,
    pub user_avatar_url: Option<String>,
    pub like_count: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateCommentParams {
    pub user_id: i32,
    pub quiz_id: i32,
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UpdateCommentParams {
    pub id: i32,
    pub content: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PaginateCommentParams {
    pub quiz_id: i32,
    pub page: i32,
    pub limit: i32,
    pub sort_by: Option<String>,
}
