use serde::{Deserialize, Serialize};
use crate::domain::models::quiz::{Quiz, QuizDifficulty};

// --- Query Structs ---

#[derive(Debug, Deserialize)]
pub struct ListQuizzesQuery {
    #[serde(rename = "categoryId")]
    pub category_id: Option<i32>,
    pub difficulty: Option<String>,
    pub q: Option<String>, // Search query
    pub page: Option<i64>,
    pub limit: Option<i64>,
}


// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateQuizDto {
    pub title: String,
    pub description: Option<String>,
    pub difficulty: QuizDifficulty,
    #[serde(rename = "categoryId")]
    pub category_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateQuizDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<QuizDifficulty>,
    #[serde(rename = "categoryId")]
    pub category_id: Option<i32>,
}


// --- Response DTOs ---

#[derive(Debug, Serialize)]
pub struct QuizDto {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub difficulty: QuizDifficulty,
    #[serde(rename = "categoryId")]
    pub category_id: Option<i32>,
    #[serde(rename = "creatorId")]
    pub creator_id: Option<i32>,
    #[serde(rename = "createdAt")]
    pub created_at: String,
}

impl From<Quiz> for QuizDto {
    fn from(quiz: Quiz) -> Self {
        Self {
            id: quiz.qz_id,
            title: quiz.qz_title,
            description: quiz.qz_description,
            difficulty: quiz.qz_difficulty,
            category_id: quiz.qz_category_id,
            creator_id: quiz.qz_creator_id,
            created_at: quiz.qz_created_at.map_or_else(String::new, |d| d.to_rfc3339()),
        }
    }
}
