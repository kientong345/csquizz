use chrono::{DateTime, Utc};
use serde_json::Value;
use sqlx::FromRow;

use crate::domain::question::model::Question;

#[derive(Debug, Clone)]
pub struct UserChoices {
    pub choices: Vec<UserChoice>,
}

#[derive(Debug, Clone)]
pub struct UserChoice {
    pub option_index: i32,
}

#[derive(Debug, Clone)]
pub struct UserEntry {
    pub entry: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct Answer {
    pub id: i32,
    pub result_id: i32,
    pub question_id: i32,
    #[sqlx(json)]
    pub data: Value,
}

#[derive(Debug, Clone, FromRow)]
pub struct AnswerDetail {
    pub answer: Answer,
    pub question: Question,
}

#[derive(Debug, Clone, FromRow)]
pub struct SubmissionResult {
    pub id: i32,
    pub user_id: i32,
    pub quiz_id: i32,
    pub score: f32,
    pub submitted_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct SubmissionResultMinimal {
    pub id: i32,
    pub score: f32,
    pub submitted_at: Option<DateTime<Utc>>,
    pub quiz_title: String,
}

#[derive(Debug, Clone, FromRow)]
pub struct SubmissionResultDetail {
    pub submission_result: SubmissionResult,
    pub detailed_answers: Vec<AnswerDetail>,
    pub quiz_title: String,
    pub completer_name: String,
    pub question_count: i64,
    pub correct_count: i64,
}

#[derive(Debug, Clone, FromRow)]
pub struct CreateSubmissionResult {
    pub user_id: i32,
    pub quiz_id: i32,
    pub score: f32,
    pub answers: Vec<Answer>,
}

#[derive(Debug, Clone, FromRow)]
pub struct SubmissionResultQuery {
    pub user_id: i32,
    pub page: u32,
    pub limit: u32,
}
