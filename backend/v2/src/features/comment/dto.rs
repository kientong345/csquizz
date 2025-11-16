use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::comment::model::{
    CommentDetail, CommentQuery, CommentSortField, CreateCommentParams, UpdateCommentParams,
};

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CommentDetailDto {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = 42)]
    pub user_id: i32,

    #[schema(example = 101)]
    pub quiz_id: i32,

    #[schema(example = "This was a great quiz!")]
    pub content: String,

    #[schema(example = "2024-04-27T12:34:56Z")]
    pub created_at: Option<String>,

    #[schema(example = "JohnDoe")]
    pub user_display_name: String,

    #[schema(example = "https://example.com/avatar.png")]
    pub user_avatar_url: Option<String>,

    #[schema(example = 10)]
    pub like_count: i64,
}

impl From<CommentDetail> for CommentDetailDto {
    fn from(model: CommentDetail) -> Self {
        Self {
            id: model.comment.id,
            user_id: model.comment.user_id,
            quiz_id: model.comment.quiz_id,
            content: model.comment.content,
            created_at: model.comment.created_at.map(|dt| dt.to_rfc3339()),
            user_display_name: model.user_display_name,
            user_avatar_url: model.user_avatar_url,
            like_count: model.like_count,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct CreateCommentDto {
    #[schema(example = 42)]
    pub user_id: i32,

    #[schema(example = 101)]
    pub quiz_id: i32,

    #[schema(example = "This was a great quiz!")]
    pub content: String,
}

impl From<CreateCommentDto> for CreateCommentParams {
    fn from(dto: CreateCommentDto) -> Self {
        Self {
            user_id: dto.user_id,
            quiz_id: dto.quiz_id,
            content: dto.content,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct UpdateCommentDto {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "This was a very helpful quiz, thank you!")]
    pub content: String,
}

impl From<UpdateCommentDto> for UpdateCommentParams {
    fn from(dto: UpdateCommentDto) -> Self {
        Self {
            id: dto.id,
            content: dto.content,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct CommentQueryDto {
    #[schema(example = 101)]
    pub quiz_id: i32,

    #[schema(example = 1)]
    pub page: i32,

    #[schema(example = 20)]
    pub limit: i32,

    #[schema(example = "MostLikes")]
    pub sort_by: Option<String>,
}

impl From<CommentQueryDto> for CommentQuery {
    fn from(dto: CommentQueryDto) -> Self {
        let sort_by = match dto.sort_by.as_deref() {
            Some("MostLikes") => CommentSortField::MostLikes,
            Some("Latest") => CommentSortField::Latest,
            _ => CommentSortField::Latest,
        };

        Self {
            quiz_id: dto.quiz_id,
            page: dto.page,
            limit: dto.limit,
            sort_by,
        }
    }
}
