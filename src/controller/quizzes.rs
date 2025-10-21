use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::{
    controller::error::ControllerError, database::pool::QuizBankPool, models::{
        pagination::{PageQuery, Paginate},
        question::{paginate::QuestionQuery, Question},
        quiz::{paginate::QuizQuery, QuizInfo},
        submission::{PostQuiz, Submission},
    }
};

pub async fn get_quizzes(
    State(pool): State<QuizBankPool>,
    Query(query): Query<QuizQuery>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = pool.get_connection().await?;

    let page = QuizInfo::page(&query, &mut *connection).await?;
    
    Ok(Json(json!(page)))
}

pub async fn get_quiz_info(
    State(pool): State<QuizBankPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = pool.get_connection().await?;

    let id: i32 = id.parse().unwrap_or(-1);
    let quiz = QuizInfo::get_by_id(id, &mut *connection).await?;
    Ok(Json(json!(quiz)))
}

pub async fn get_question_page(
    State(pool): State<QuizBankPool>,
    Path(id): Path<String>,
    Query(query): Query<PageQuery>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = pool.get_connection().await?;

    let quiz_id: i32 = id.parse().unwrap_or(-1);
    let query = QuestionQuery {
        quiz_id,
        page: query.page,
        size: query.size,
    };

    let page = Question::page(&query, &mut *connection).await?;
    Ok(Json(json!(page)))
}

pub async fn submit_quiz(
    State(pool): State<QuizBankPool>,
    Path(id): Path<String>,
    Json(submission): Json<Submission>,
) -> Result<Json<Value>, ControllerError> {
    if id.parse().unwrap_or(-1) != submission.user_id {
        // return Err(StatusCode::BAD_REQUEST);
    }
    let submission_result = submission.evaluate();
    let mut connection = pool.get_connection().await?;
    
    submission_result.store(&mut *connection).await?;
    Ok(Json(json!(submission_result.summary.id)))
}

pub async fn create_quiz(
    State(pool): State<QuizBankPool>,
    Json(data): Json<PostQuiz>,
) -> Result<(), ControllerError> {
    let mut connection = pool.start_transaction().await?;

    let new_quiz = QuizInfo::create(data.info, &mut connection).await?;

    Question::create(new_quiz.id, data.questions, &mut connection).await?;

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
