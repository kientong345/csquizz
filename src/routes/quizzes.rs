use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    database::pool::QuizBankPool,
    services::quizzes::{get_quiz, get_quizzes, submit_quiz},
};

pub fn create_route(pool: QuizBankPool) -> Router {
    Router::new()
        .route("/api/quizzes", get(get_quizzes))
        .route("/api/quizzes/:id", get(get_quiz))
        .route("/api/quizzes/:id/submit", post(submit_quiz))
        .with_state(pool)
}
