use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
};
use serde_json::{Value, json};
use tokio::sync::RwLock;

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        pagination::Paginate,
        question::Question,
        quiz::{QuizMetadata, paginate::QuizQuery},
        submission::PostQuiz,
    },
};

pub async fn paginate(
    State(state): State<Arc<RwLock<AppState>>>,
    Query(query): Query<QuizQuery>,
) -> Result<Json<Value>, ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.get_connection().await?;

    let page = QuizMetadata::page(&query, &mut *connection).await?;

    Ok(Json(json!(page)))
}

pub async fn get(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.get_connection().await?;

    let id: i32 = id.parse().unwrap_or(-1);
    let quiz = QuizMetadata::get_by_id(id, &mut *connection).await?;
    Ok(Json(json!(quiz)))
}

pub async fn post(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(data): Json<PostQuiz>,
) -> Result<(), ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.start_transaction().await?;

    QuizMetadata::create_from(data.metadata, &mut connection).await?;

    for question in data.questions {
        Question::create_from(question, &mut connection).await?;
    }

    connection.commit().await?;

    Ok(())
}

pub async fn update(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<Json<Value>, ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.get_connection().await?;

    todo!()
}

pub async fn delete(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<Json<Value>, ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.get_connection().await?;

    todo!()
}

pub async fn add_question(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<Json<Value>, ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.get_connection().await?;

    todo!()
}

pub async fn update_question(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<Json<Value>, ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.get_connection().await?;

    todo!()
}

pub async fn delete_question(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<Json<Value>, ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.get_connection().await?;

    todo!()
}
