use serde::Deserialize;

use crate::models::comment::{CommentCreateParams, CommentPaginateParams};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommentPaginateParamsDto {
    pub page: i32,
    pub page_size: i32,
    pub sort_by: Option<String>,
}

impl CommentPaginateParamsDto {
    pub fn bind(self, quiz_id: i32) -> CommentPaginateParams {
        CommentPaginateParams {
            quiz_id,
            page: self.page,
            page_size: self.page_size,
            sort_by: self.sort_by,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CommentCreateParamsDto {
    pub content: String,
}

impl CommentCreateParamsDto {
    pub fn bind(self, user_id: i32, quiz_id: i32) -> CommentCreateParams {
        CommentCreateParams {
            user_id,
            quiz_id,
            content: self.content,
        }
    }
}
