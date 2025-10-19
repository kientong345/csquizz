use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde_json::Value;

use crate::{database::pool::QuizBankPool, models::result::paginate::QuestionAnswerResultQuery};

pub async fn get_results(
    State(pool): State<QuizBankPool>,
    Query(query): Query<QuestionAnswerResultQuery>,
) -> Result<Json<Value>, StatusCode> {
    todo!()
}
