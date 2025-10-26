use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::{json, Value};

use crate::{
    controller::error::ControllerError,
    database::pool::QuizBankPool,
    models::{
        category::{paginate::CategoryQuery, Category},
        pagination::Paginate,
    },
};

pub async fn get_categories(
    State(pool): State<QuizBankPool>,
    Query(query): Query<CategoryQuery>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = pool.get_connection().await?;

    let page = Category::page(&query, &mut *connection).await?;

    Ok(Json(json!(page)))
}
