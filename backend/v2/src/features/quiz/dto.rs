use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::quiz::model::{QuizDetail, QuizMinimal, QuizQuery};

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QuizMinimalDto {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "Introduction to Rust")]
    pub title: String,

    #[schema(example = "Beginner")]
    pub difficulty: Option<String>,

    #[schema(example = 10)]
    pub question_count: i64,

    #[schema(example = 100)]
    pub like_count: i64,

    #[schema(example = "Programming Languages")]
    pub category_name: String,
}

impl From<QuizMinimal> for QuizMinimalDto {
    fn from(quiz_minimal: QuizMinimal) -> Self {
        QuizMinimalDto {
            id: quiz_minimal.id,
            title: quiz_minimal.title,
            difficulty: quiz_minimal.difficulty.map(|d| d.to_string()),
            question_count: quiz_minimal.question_count,
            like_count: quiz_minimal.like_count,
            category_name: quiz_minimal.category_name,
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct QuizDetailDto {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "Introduction to Rust")]
    pub title: String,

    #[schema(example = "A quiz about the Rust programming language.")]
    pub description: Option<String>,

    #[schema(example = "Beginner")]
    pub difficulty: Option<String>,

    #[schema(example = 2)]
    pub category_id: Option<i32>,

    #[schema(example = 3)]
    pub creator_id: Option<i32>,

    #[schema(example = "2024-01-01T12:00:00Z")]
    pub created_at: Option<String>,

    #[schema(example = "2024-01-02T12:00:00Z")]
    pub updated_at: Option<String>,

    #[schema(example = 10)]
    pub question_count: i64,

    #[schema(example = 100)]
    pub like_count: i64,

    #[schema(example = 5)]
    pub comment_count: i64,

    #[schema(example = "Programming Languages")]
    pub category_name: String,
}

impl From<QuizDetail> for QuizDetailDto {
    fn from(value: QuizDetail) -> Self {
        QuizDetailDto {
            id: value.quiz.id,
            title: value.quiz.title,
            description: value.quiz.description,
            difficulty: value.quiz.difficulty.map(|d| d.to_string()),
            category_id: value.quiz.category_id,
            creator_id: value.quiz.creator_id,
            created_at: value.quiz.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: value.quiz.updated_at.map(|dt| dt.to_rfc3339()),
            question_count: value.question_count,
            like_count: value.like_count,
            comment_count: value.comment_count,
            category_name: value.category_name,
        }
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct CreateQuizParamsDto {
    #[schema(example = "Introduction to Rust")]
    pub title: String,

    #[schema(example = "A quiz about the Rust programming language.")]
    pub description: Option<String>,

    #[schema(example = "Beginner")]
    pub difficulty: Option<String>,

    #[schema(example = 2)]
    pub category_id: i32,

    #[schema(example = 3)]
    pub creator_id: i32,
}

impl From<CreateQuizParamsDto> for crate::domain::quiz::model::CreateQuizParams {
    fn from(dto: CreateQuizParamsDto) -> Self {
        crate::domain::quiz::model::CreateQuizParams {
            title: dto.title,
            description: dto.description,
            difficulty: dto.difficulty.and_then(|d| d.parse().ok()),
            category_id: dto.category_id,
            creator_id: dto.creator_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct UpdateQuizParamsDto {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "Advanced Rust Concepts")]
    pub title: Option<String>,

    #[schema(example = "An advanced quiz about Rust.")]
    pub description: Option<String>,

    #[schema(example = "Advanced")]
    pub difficulty: Option<String>,

    #[schema(example = 3)]
    pub category_id: Option<i32>,
}

impl From<UpdateQuizParamsDto> for crate::domain::quiz::model::UpdateQuizParams {
    fn from(dto: UpdateQuizParamsDto) -> Self {
        crate::domain::quiz::model::UpdateQuizParams {
            id: dto.id,
            title: dto.title,
            description: dto.description,
            difficulty: dto.difficulty.and_then(|d| d.parse().ok()),
            category_id: dto.category_id,
        }
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct QuizQueryDto {
    #[schema(example = "Rust")]
    pub title_pattern: Option<String>,

    #[schema(example = 2)]
    pub category_id: Option<i32>,

    #[schema(example = "Beginner")]
    pub difficulty: Option<String>,

    #[schema(example = 1)]
    pub page: u32,

    #[schema(example = 10)]
    pub limit: u32,

    #[schema(example = "title")]
    pub sort_by: String,
}

impl From<QuizQueryDto> for QuizQuery {
    fn from(value: QuizQueryDto) -> Self {
        QuizQuery {
            title_pattern: value.title_pattern,
            category_id: value.category_id,
            difficulty: value.difficulty.and_then(|d| d.parse().ok()),
            page: value.page,
            limit: value.limit,
            sort_by: value
                .sort_by
                .parse()
                .unwrap_or(crate::domain::quiz::model::QuizSortField::Title),
        }
    }
}
