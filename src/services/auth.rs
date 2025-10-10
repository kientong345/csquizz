use axum::{extract::State, http::StatusCode, Json};
use serde_json::Value;

use crate::{
    database::pool::QuizBankPool,
    models::auth::{Logination, Registration},
};

pub async fn handle_register(
    State(pool): State<QuizBankPool>,
    Json(data): Json<Registration>,
) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn handle_login(
    State(pool): State<QuizBankPool>,
    Json(data): Json<Logination>,
) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn get_my_info(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}
