use axum::{
    Extension, Json,
    extract::{Path, Query, State},
};
use reqwest::StatusCode;
use serde_json::{Value, json};

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        answer::Answer,
        auth::AccessClaims,
        comment::{CommentDetail, DatabaseComment},
        input_dto::{
            comment::{CommentCreateParamsDto, CommentPaginateParamsDto},
            question::{
                QuestionCreateParamsDto, QuestionPaginateParamsDto, QuestionUpdateParamsDto,
            },
            quiz::QuizUpdateParamsDto,
            quiz_question::QuizQuestionCreateParamsDto,
            submission::QuizSubmissionParamsDto,
        },
        like::DatabaseQuizLike,
        pagination::Paginate,
        question::{DatabaseQuestion, QuestionPrivateData, QuestionPublicData},
        quiz::{DatabaseQuiz, QuizDetail, QuizMinimal, QuizPaginateParams},
        quiz_composition::{QuizComment, QuizPublicQuestion},
        submission_result::DatabaseSubmissionResult,
    },
    services::evaluator::Evaluator,
};

pub async fn get_quizzes_page(
    State(state): State<AppState>,
    Query(params): Query<QuizPaginateParams>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    let page = QuizMinimal::page(&params, &mut *connection).await?;

    connection.commit().await?;

    Ok(Json(json!(page)))
}

pub async fn get_quiz_with_comments(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Query(params): Query<CommentPaginateParamsDto>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    let metadata = QuizDetail::get_by_id(id, &mut *connection).await?;
    let data = CommentDetail::page(&params.bind(id), &mut *connection).await?;

    connection.commit().await?;

    Ok(Json(json!(QuizComment { metadata, data })))
}

pub async fn get_quiz_with_questions(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Query(params): Query<QuestionPaginateParamsDto>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    let metadata = QuizDetail::get_by_id(id, &mut *connection).await?;
    let data = QuestionPrivateData::page(&params.bind(id), &mut *connection)
        .await?
        .try_map_into::<QuestionPublicData>()?;

    connection.commit().await?;

    Ok(Json(json!(QuizPublicQuestion { metadata, data })))
}

pub async fn create_quiz_with_questions(
    State(state): State<AppState>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<QuizQuestionCreateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);

    let mut connection = state.primary_db.start_transaction().await?;

    let quiz_id = DatabaseQuiz::create_from(&payload.quiz_params.bind(user_id), &mut *connection)
        .await?
        .id;

    for params in payload.questions_params {
        DatabaseQuestion::create_from(&params.bind(quiz_id), &mut *connection).await?;
    }

    connection.commit().await?;

    Ok(StatusCode::CREATED)
}

pub async fn update_quiz_metadata(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<QuizUpdateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    DatabaseQuiz::update_by(&payload.bind(id), &mut *connection).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn delete_quiz(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    DatabaseQuiz::delete_by(id, &mut *connection).await?;
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

    DatabaseQuizLike::create_from(user_id, id, &mut *connection).await?;

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

    DatabaseComment::create_from(&payload.bind(user_id, id), &mut *connection).await?;

    connection.commit().await?;

    Ok(StatusCode::CREATED)
}

pub async fn add_question(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<QuestionCreateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    DatabaseQuestion::create_from(&payload.bind(id), &mut *connection).await?;

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

    DatabaseQuestion::update_by(&payload.bind(question_id), &mut *connection).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn delete_question(
    State(state): State<AppState>,
    Path(_id): Path<i32>,
    Path(question_id): Path<i32>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    DatabaseQuestion::delete_by(question_id, &mut *connection).await?;

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

    let submission = payload.bind(user_id, id);

    let mut connection = state.primary_db.start_transaction().await?;

    let (submission_result_summary, evaluated_answers) =
        Evaluator::evaluate(&submission, &mut *connection).await?;

    let result_id =
        DatabaseSubmissionResult::create_from(&submission_result_summary, &mut *connection)
            .await?
            .id;

    for evaluated_answer in evaluated_answers {
        Answer::create_from(&evaluated_answer.bind(result_id), &mut *connection).await?;
    }

    connection.commit().await?;

    Ok(StatusCode::OK)
}
