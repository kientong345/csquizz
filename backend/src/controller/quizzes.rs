use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::{
    controller::error::ControllerError,
    database::pool::QuizBankPool,
    models::{
        pagination::Paginate,
        question::{paginate::QuestionQuery, Question, QuestionNoKey},
        quiz::{paginate::QuizQuery, QuizMetadata},
        submission::{PostQuiz, SubmittedQuiz},
    },
};

pub async fn get_quizzes(
    State(pool): State<QuizBankPool>,
    Query(query): Query<QuizQuery>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = pool.get_connection().await?;

    let page = QuizMetadata::page(&query, &mut *connection).await?;

    Ok(Json(json!(page)))
}

pub async fn get_quiz_by_id(
    State(pool): State<QuizBankPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = pool.get_connection().await?;

    let id: i32 = id.parse().unwrap_or(-1);
    let quiz = QuizMetadata::get_by_id(id, &mut *connection).await?;
    Ok(Json(json!(quiz)))
}

pub async fn get_questions(
    State(pool): State<QuizBankPool>,
    Query(query): Query<QuestionQuery>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = pool.get_connection().await?;
    let page = QuestionNoKey::page(&query, &mut *connection).await?;
    Ok(Json(json!(page)))
}

pub async fn submit_quiz(
    State(pool): State<QuizBankPool>,
    Json(submission): Json<SubmittedQuiz>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = pool.start_transaction().await?;

    let tmp_quiz_result = submission
        .evaluate(&mut *connection)
        .await?
        .into_tmp_quiz_result(&mut *connection)
        .await?;

    connection.commit().await?;

    Ok(Json(json!(tmp_quiz_result)))
}

pub async fn submit_quiz_and_store_result(
    State(pool): State<QuizBankPool>,
    Json(submission): Json<SubmittedQuiz>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = pool.start_transaction().await?;

    let quiz_result = submission
        .evaluate(&mut *connection)
        .await?
        .into_quiz_result(&mut *connection)
        .await?;

    connection.commit().await?;

    Ok(Json(json!(quiz_result)))
}

pub async fn create_quiz(
    State(pool): State<QuizBankPool>,
    Json(data): Json<PostQuiz>,
) -> Result<(), ControllerError> {
    let mut connection = pool.start_transaction().await?;

    QuizMetadata::create_from(data.metadata, &mut connection).await?;

    for question in data.questions {
        Question::create_from(question, &mut connection).await?;
    }

    connection.commit().await?;

    Ok(())
}

pub async fn update_quiz_info(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn delete_quiz(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn add_question(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn update_question(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn delete_question(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}
