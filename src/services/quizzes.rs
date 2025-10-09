use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::{
    database::pool::QuizBankPool,
    models::{
        paginate::{PageQuery, Paginate},
        question::{Question, QuestionQuery},
        quiz::{QuizInfo, QuizQuery},
    },
};

pub async fn get_quizzes(
    State(pool): State<QuizBankPool>,
    Query(query): Query<QuizQuery>,
) -> Result<Json<Value>, StatusCode> {
    let mut connection = match pool.get_connection().await {
        Ok(connection) => connection,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    match QuizInfo::page(&query, &mut *connection).await {
        Ok(page) => Ok(Json(json!(page))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_quiz_info(
    State(pool): State<QuizBankPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let mut connection = match pool.get_connection().await {
        Ok(connection) => connection,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let id: i32 = id.parse().unwrap_or(-1);
    match QuizInfo::get_by_id(id, &mut *connection).await {
        Ok(quiz) => Ok(Json(json!(quiz))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_question_page(
    State(pool): State<QuizBankPool>,
    Path(id): Path<String>,
    Query(query): Query<PageQuery>,
) -> Result<Json<Value>, StatusCode> {
    let mut connection = match pool.get_connection().await {
        Ok(connection) => connection,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let quiz_id: i32 = id.parse().unwrap_or(-1);
    let query = QuestionQuery {
        quiz_id,
        page: query.page,
        size: query.size,
    };

    match Question::page(&query, &mut *connection).await {
        Ok(page) => Ok(Json(json!(page))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn submit_quiz(
    State(pool): State<QuizBankPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn create_quiz(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
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
