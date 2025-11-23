use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};

use crate::models::error::ModelError;

pub mod create;
pub mod delete;
pub mod get;
pub mod paginate;
pub mod update;

#[derive(Debug, Clone, Type, PartialEq, Eq)]
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
    type Err = ModelError;

    fn from_str(input: &str) -> Result<QuizDifficulty, Self::Err> {
        match input {
            "easy" => Ok(QuizDifficulty::Easy),
            "medium" => Ok(QuizDifficulty::Medium),
            "hard" => Ok(QuizDifficulty::Hard),
            _ => Err(ModelError::BadPost(input.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum QuizSortField {
    CreatedAt,
    LikeCount,
}

impl FromStr for QuizSortField {
    type Err = ModelError;

    fn from_str(input: &str) -> Result<QuizSortField, Self::Err> {
        match input {
            "created_at" => Ok(QuizSortField::CreatedAt),
            "like_count" => Ok(QuizSortField::LikeCount),
            _ => Err(ModelError::BadPost(input.to_string())),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct DatabaseQuiz {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub difficulty: Option<QuizDifficulty>,
    pub category_id: Option<i32>,
    pub creator_id: Option<i32>,
    pub pass_score: f64,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizMinimal {
    pub id: i32,
    pub title: String,
    pub difficulty: Option<String>,
    pub question_count: i64,
    pub like_count: i64,
    pub category_name: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizDetail {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub category_id: Option<i32>,
    pub creator_id: Option<i32>,
    pub pass_score: f64,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub question_count: i64,
    pub like_count: i64,
    pub comment_count: i64,
    pub category_name: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuizCreateParams {
    pub title: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub category_id: i32,
    pub creator_id: i32,
    pub pass_score: f64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuizUpdateParams {
    pub id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub category_id: Option<i32>,
    pub pass_score: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuizPaginateParams {
    pub title_pattern: Option<String>,
    pub category_id: Option<i32>,
    pub difficulty: Option<String>,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: String,
}
