use std::str::FromStr;

use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Enum đại diện cho độ khó của một bài quiz.
#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "quiz_difficulty", rename_all = "snake_case")]
pub enum QuizDifficulty {
    Easy,
    Medium,
    Hard,
}

impl ToString for QuizDifficulty {
    fn to_string(&self) -> String {
        match self {
            QuizDifficulty::Easy => "easy".to_string(),
            QuizDifficulty::Medium => "medium".to_string(),
            QuizDifficulty::Hard => "hard".to_string(),
        }
    }
}

impl FromStr for QuizDifficulty {
    type Err = ();

    fn from_str(input: &str) -> Result<QuizDifficulty, Self::Err> {
        match input {
            "easy" => Ok(QuizDifficulty::Easy),
            "medium" => Ok(QuizDifficulty::Medium),
            "hard" => Ok(QuizDifficulty::Hard),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QuizSortField {
    CreatedAt,
    LikeCount,
    CommentCount,
    QuestionCount,
    Title,
}

impl FromStr for QuizSortField {
    type Err = ();

    fn from_str(input: &str) -> Result<QuizSortField, Self::Err> {
        match input {
            "created_at" => Ok(QuizSortField::CreatedAt),
            "like_count" => Ok(QuizSortField::LikeCount),
            "comment_count" => Ok(QuizSortField::CommentCount),
            "question_count" => Ok(QuizSortField::QuestionCount),
            "title" => Ok(QuizSortField::Title),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct Quiz {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub difficulty: Option<QuizDifficulty>,
    pub category_id: Option<i32>,
    pub creator_id: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct QuizMinimal {
    pub id: i32,
    pub title: String,
    pub difficulty: Option<QuizDifficulty>,
    pub question_count: i64,
    pub like_count: i64,
    pub category_name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct QuizDetail {
    pub quiz: Quiz,
    pub question_count: i64,
    pub like_count: i64,
    pub comment_count: i64,
    pub category_name: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct CreateQuizParams {
    pub title: String,
    pub description: Option<String>,
    pub difficulty: Option<QuizDifficulty>,
    pub category_id: i32,
    pub creator_id: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct UpdateQuizParams {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<QuizDifficulty>,
    pub category_id: Option<i32>,
}

#[derive(Debug, Clone, FromRow)]
pub struct QuizQuery {
    pub title_pattern: Option<String>,
    pub category_id: Option<i32>,
    pub difficulty: Option<QuizDifficulty>,
    pub page: u32,
    pub limit: u32,
    pub sort_by: QuizSortField,
}
