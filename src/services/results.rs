use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;

use crate::{database::pool::QuizBankPool, models::result::QuizResultQuery};

pub async fn get_results(
    State(pool): State<QuizBankPool>,
    Query(query): Query<QuizResultQuery>,
) -> Result<Json<Value>, StatusCode> {
    todo!()
}
