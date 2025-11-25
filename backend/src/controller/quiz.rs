use axum::{
    Extension, Json,
    extract::{Path, Query, State},
};
use reqwest::StatusCode;
use serde_json::Value;

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        auth::AccessClaims,
        input_dto::{
            comment::{CommentCreateParamsDto, CommentPaginateParamsDto},
            question::{
                QuestionCreateParamsDto, QuestionPaginateParamsDto, QuestionUpdateParamsDto,
            },
            quiz::QuizUpdateParamsDto,
            quiz_question::QuizQuestionCreateParamsDto,
            submission::QuizSubmissionParamsDto,
        },
        quiz::QuizPaginateParams,
    },
};

pub async fn get_quizzes_page(
    State(state): State<AppState>,
    Query(params): Query<QuizPaginateParams>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    let page = state
        .quiz_service
        .get_quizzes_page(&mut *connection, &params)
        .await?;

    connection.commit().await?;

    Ok(Json(page))
}

pub async fn get_quiz_with_comments(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Query(params): Query<CommentPaginateParamsDto>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    let data = state
        .quiz_service
        .get_quiz_with_comments(&mut *connection, id, &params)
        .await?;

    connection.commit().await?;

    Ok(Json(data))
}

pub async fn get_quiz_with_questions(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Query(params): Query<QuestionPaginateParamsDto>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    let data = state
        .quiz_service
        .get_quiz_with_questions(&mut *connection, id, &params)
        .await?;

    connection.commit().await?;

    Ok(Json(data))
}

pub async fn create_quiz_with_questions(
    State(state): State<AppState>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<QuizQuestionCreateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);

    let mut connection = state.primary_db.start_transaction().await?;

    state
        .quiz_service
        .create_quiz_with_questions(&mut *connection, user_id, &payload)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::CREATED)
}

pub async fn update_quiz_metadata(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<QuizUpdateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    state
        .quiz_service
        .update_quiz_metadata(&mut *connection, id, &payload)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn delete_quiz(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    state.quiz_service.delete_quiz(&mut *connection, id).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn like_quiz(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(access_claims): Extension<AccessClaims>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);

    let mut connection = state.primary_db.get_connection().await?;

    state
        .quiz_service
        .like_quiz(&mut *connection, user_id, id)
        .await?;

    Ok(StatusCode::CREATED)
}

pub async fn comment_quiz(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<CommentCreateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);

    let mut connection = state.primary_db.start_transaction().await?;

    state
        .quiz_service
        .comment_quiz(&mut *connection, user_id, id, &payload)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::CREATED)
}

pub async fn add_question(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<QuestionCreateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    state
        .quiz_service
        .add_question(&mut *connection, id, &payload)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::CREATED)
}

pub async fn update_question(
    State(state): State<AppState>,
    Path(_id): Path<i32>,
    Path(question_id): Path<i32>,
    Json(payload): Json<QuestionUpdateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    state
        .quiz_service
        .update_question(&mut *connection, question_id, &payload)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn delete_question(
    State(state): State<AppState>,
    Path(_id): Path<i32>,
    Path(question_id): Path<i32>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    state
        .quiz_service
        .delete_question(&mut *connection, question_id)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn submit_quiz(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<QuizSubmissionParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);

    let mut connection = state.primary_db.start_transaction().await?;

    state
        .quiz_service
        .submit_quiz(&mut *connection, user_id, id, &payload)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}
