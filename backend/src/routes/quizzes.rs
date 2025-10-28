use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    controller::quizzes::{get_questions, get_quiz_by_id, get_quizzes, submit_quiz},
    database::pool::QuizBankPool,
};

pub fn create_route(pool: QuizBankPool) -> Router {
    Router::new()
        .route("/api/quizzes", get(get_quizzes))
        .route("/api/quizzes/{:id}", get(get_quiz_by_id))
        .route("/api/quizzes/{:id}/submit", post(submit_quiz))
        .route("/api/questions", get(get_questions))
        .with_state(pool)
}
