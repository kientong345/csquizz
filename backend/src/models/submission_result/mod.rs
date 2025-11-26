use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::error::ModelError;

pub mod create;
pub mod get;
pub mod paginate;

#[derive(Debug, Clone, Copy)]
pub enum SubmissionResultSortField {
    LatestSubmission,
    HighestScore,
}

impl FromStr for SubmissionResultSortField {
    type Err = ModelError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "latest" => Ok(SubmissionResultSortField::LatestSubmission),
            "highest" => Ok(SubmissionResultSortField::HighestScore),
            _ => Err(ModelError::BadPost(s.to_string())),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct DatabaseSubmissionResult {
    pub id: i32,
    pub user_id: i32,
    pub quiz_id: i32,
    pub score: f32,
    pub is_passed: bool,
    pub submitted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionResultMinimal {
    pub id: i32,
    pub score: f32,
    pub is_passed: bool,
    pub submitted_at: Option<String>,
    pub quiz_title: String,
}

#[derive(Debug, Clone, Serialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionResultDetail {
    pub id: i32,
    pub user_id: i32,
    pub quiz_id: i32,
    pub score: f32,
    pub is_passed: bool,
    pub submitted_at: Option<String>,
    pub quiz_title: String,
    pub owner_name: String,
    pub question_count: i64,
    pub answer_count: i64,
    pub correct_count: i64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionResultCreateParams {
    pub user_id: i32,
    pub quiz_id: i32,
    pub score: f64,
    pub is_passed: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionResultPaginateParams {
    pub user_id: i32,
    pub quiz_title_pattern: String,
    pub passed_only: bool,
    pub quiz_difficulty: Option<String>,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: String,
}
