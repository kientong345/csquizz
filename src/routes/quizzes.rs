use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    controller::quizzes::{get_question_page, get_quiz_info, get_quizzes, submit_quiz},
    database::pool::QuizBankPool,
};

pub fn create_route(pool: QuizBankPool) -> Router {
    Router::new()
        .route("/api/quizzes", get(get_quizzes))
        .route("/api/quizzes/{:id}", get(get_quiz_info))
        .route("/api/quizzes/{:id}/submit", post(submit_quiz))
        .route("/api/questions", get(get_question_page))
        .with_state(pool)
}
