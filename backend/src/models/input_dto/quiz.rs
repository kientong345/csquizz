use serde::Deserialize;

use crate::models::quiz::{QuizCreateParams, QuizUpdateParams};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizCreateParamsDto {
    pub title: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub category_id: i32,
    pub pass_score: f64,
}

impl QuizCreateParamsDto {
    pub fn bind(self, creator_id: i32) -> QuizCreateParams {
        QuizCreateParams {
            title: self.title,
            description: self.description,
            difficulty: self.difficulty,
            category_id: self.category_id,
            creator_id,
            pass_score: self.pass_score,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizUpdateParamsDto {
    pub title: Option<String>,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub category_id: Option<i32>,
    pub pass_score: Option<f64>,
}

impl QuizUpdateParamsDto {
    pub fn bind(self, id: i32) -> QuizUpdateParams {
        QuizUpdateParams {
            id,
            title: self.title,
            description: self.description,
            difficulty: self.difficulty,
            category_id: self.category_id,
            pass_score: self.pass_score,
        }
    }
}
