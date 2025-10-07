use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;

use crate::{database::pool::QuizBankPool, models::quiz::QuizQuery};

pub async fn get_quizzes(
    State(pool): State<QuizBankPool>,
    Query(query): Query<QuizQuery>,
) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn get_quiz(
    State(pool): State<QuizBankPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn submit_quiz(
    State(pool): State<QuizBankPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    todo!()
}
