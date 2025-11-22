use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde_json::{Value, json};

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        pagination::Paginate,
        quiz::{DatabaseQuiz, QuizCreateParams, QuizDetail, QuizMinimal, QuizPaginateParams},
    },
};

pub async fn paginate(
    State(state): State<AppState>,
    Query(query): Query<QuizPaginateParams>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    let page = QuizMinimal::page(&query, &mut *connection).await?;

    Ok(Json(json!(page)))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    let id: i32 = id.parse().unwrap_or(-1);
    let quiz = QuizDetail::get_by_id(id, &mut *connection).await?;
    Ok(Json(json!(quiz)))
}

pub async fn post(
    State(state): State<AppState>,
    Json(data): Json<QuizCreateParams>,
) -> Result<(), ControllerError> {
    // let mut connection = state.pool.start_transaction().await?;

    // DatabaseQuiz::create_from(data.metadata, &mut connection).await?;

    // for question in data.questions {
    //     Question::create_from(question, &mut connection).await?;
    // }

    // connection.commit().await?;

    Ok(())
}

pub async fn update(State(state): State<AppState>) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    todo!()
}

pub async fn delete(State(state): State<AppState>) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    todo!()
}

pub async fn add_question(State(state): State<AppState>) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    todo!()
}

pub async fn update_question(
    State(state): State<AppState>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    todo!()
}

pub async fn delete_question(
    State(state): State<AppState>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    todo!()
}
