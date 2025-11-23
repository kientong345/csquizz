use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::error::ModelError;

pub mod create;
pub mod delete;
pub mod get;
pub mod paginate;
pub mod update;

#[derive(Debug, Clone, Copy)]
pub enum CommentSortField {
    Latest,
    MostLike,
}

impl FromStr for CommentSortField {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "latest" => Ok(CommentSortField::Latest),
            "most-like" => Ok(CommentSortField::MostLike),
            _ => Err(ModelError::BadPost(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct DatabaseComment {
    pub id: i32,
    pub user_id: i32,
    pub quiz_id: i32,
    pub content: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommentCreateParams {
    pub user_id: i32,
    pub quiz_id: i32,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommentUpdateParams {
    pub id: i32,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommentPaginateParams {
    pub quiz_id: i32,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: String,
}
