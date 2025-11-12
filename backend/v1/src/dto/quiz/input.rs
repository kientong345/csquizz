use serde::Deserialize;

use crate::models::quiz::{QuizDifficulty, paginate::QuizQuery, post::PostQuizMetadata};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuizQueryDto {
    pub category_id: Option<i32>,
    pub title_pattern: Option<String>,
    pub difficulty: Option<String>,
    pub created_by: Option<i32>,
    pub completed_by: Option<i32>,
    pub page: i64,
    pub size: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostQuizMetadataDto {
    pub title: String,
    pub description: Option<String>,
    pub category_id: i32,
    pub difficulty: Option<String>,
    pub creator_id: Option<i32>,
}

impl From<QuizQueryDto> for QuizQuery {
    fn from(value: QuizQueryDto) -> Self {
        Self {
            category_id: value.category_id,
            title_pattern: value.title_pattern,
            difficulty: value.difficulty,
            created_by: value.created_by,
            completed_by: value.completed_by,
            page: value.page,
            size: value.size,
        }
    }
}

impl From<PostQuizMetadataDto> for PostQuizMetadata {
    fn from(value: PostQuizMetadataDto) -> Self {
        let difficulty = if let Some(diff) = value.difficulty {
            match diff.as_ref() {
                "easy" => Some(QuizDifficulty::Easy),
                "medium" => Some(QuizDifficulty::Medium),
                "hard" => Some(QuizDifficulty::Hard),
                _ => None,
            }
        } else {
            None
        };

        Self {
            title: value.title,
            description: value.description,
            category_id: value.category_id,
            difficulty,
            creator_id: value.creator_id,
        }
    }
}
