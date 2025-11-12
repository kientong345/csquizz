use serde::Serialize;

use crate::models::quiz::{QuizDifficulty, QuizMetadata, QuizMinimal};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizMinimalDto {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub difficulty: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizMetadataDto {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub category: String,
    pub question_count: i64,
    pub difficulty: Option<String>,
    pub created_by: Option<String>,
}

impl Into<QuizMinimalDto> for QuizMinimal {
    fn into(self) -> QuizMinimalDto {
        let difficulty = match self.difficulty {
            Some(QuizDifficulty::Easy) => Some(String::from("easy")),
            Some(QuizDifficulty::Medium) => Some(String::from("medium")),
            Some(QuizDifficulty::Hard) => Some(String::from("hard")),
            _ => None,
        };

        QuizMinimalDto {
            id: self.id,
            title: self.title,
            category: self.category,
            difficulty,
        }
    }
}

impl Into<QuizMetadataDto> for QuizMetadata {
    fn into(self) -> QuizMetadataDto {
        let difficulty = match self.difficulty {
            Some(QuizDifficulty::Easy) => Some(String::from("easy")),
            Some(QuizDifficulty::Medium) => Some(String::from("medium")),
            Some(QuizDifficulty::Hard) => Some(String::from("hard")),
            _ => None,
        };

        QuizMetadataDto {
            id: self.id,
            title: self.title,
            description: self.description,
            category: self.category,
            question_count: self.question_count,
            difficulty,
            created_by: self.created_by,
        }
    }
}
