use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;

pub mod create;
pub mod get;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserChoice {
    pub option_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntry {
    pub entry: String,
}

#[derive(Debug, Clone, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Answer {
    pub id: i32,
    pub result_id: i32,
    pub question_id: i32,
    pub is_correct: bool,
    #[sqlx(json)]
    pub data: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnswerCreateParams {
    pub result_id: i32,
    pub question_id: i32,
    pub is_correct: bool,
    pub data: Value,
}
