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

pub async fn create_quiz(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn update_quiz_info(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn delete_quiz(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn add_question(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn update_question(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn delete_question(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}
