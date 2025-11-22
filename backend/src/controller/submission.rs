use axum::{Json, extract::State};
use serde_json::{Value, json};

use crate::{app::AppState, controller::error::ControllerError, models::submission::SubmittedQuiz};

pub async fn submit(
    State(state): State<AppState>,
    Json(submission): Json<SubmittedQuiz>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.pool.start_transaction().await?;

    let quiz_result = submission
        .evaluate(&mut *connection)
        .await?
        .into_quiz_result(&mut *connection)
        .await?;

    connection.commit().await?;

    Ok(Json(json!(quiz_result)))
}

pub async fn unauthorized_submit(
    State(state): State<AppState>,
    Json(submission): Json<SubmittedQuiz>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.pool.start_transaction().await?;

    let tmp_quiz_result = submission
        .evaluate(&mut *connection)
        .await?
        .into_tmp_quiz_result(&mut *connection)
        .await?;

    connection.commit().await?;

    Ok(Json(json!(tmp_quiz_result)))
}
