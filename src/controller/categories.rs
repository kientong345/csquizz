use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::{json, Value};

use crate::{
    controller::error::ControllerError,
    database::pool::QuizBankPool,
    models::{
        category::{paginate::QuizCategoryQuery, QuizCategory},
        pagination::Paginate,
    },
};

pub async fn get_categories(
    State(pool): State<QuizBankPool>,
    Query(query): Query<QuizCategoryQuery>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = pool.get_connection().await?;

    let page = QuizCategory::page(&query, &mut *connection).await?;

    Ok(Json(json!(page)))
}
