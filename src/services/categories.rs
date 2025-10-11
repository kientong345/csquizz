use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::{
    database::pool::QuizBankPool,
    models::{
        category::{QuizCategory, QuizCategoryQuery},
        pagination::Paginate,
    },
};

pub async fn get_categories(
    State(pool): State<QuizBankPool>,
    Query(query): Query<QuizCategoryQuery>,
) -> Result<Json<Value>, StatusCode> {
    let mut connection = match pool.get_connection().await {
        Ok(connection) => connection,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match QuizCategory::page(&query, &mut *connection).await {
        Ok(page) => Ok(Json(json!(page))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
