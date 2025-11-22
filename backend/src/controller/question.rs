use axum::{
    Json,
    extract::{Query, State},
};
use serde_json::{Value, json};

use crate::{app::AppState, controller::error::ControllerError, models::pagination::Paginate};

pub async fn paginate(
    State(state): State<AppState>,
    Query(query): Query<QuestionQuery>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.pool.get_connection().await?;
    let page = QuestionNoKey::page(&query, &mut *connection).await?;
    Ok(Json(json!(page)))
}
