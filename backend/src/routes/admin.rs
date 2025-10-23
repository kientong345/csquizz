use axum::{
    routing::{patch, post, put},
    Router,
};

use crate::{
    controller::quizzes::{
        add_question, create_quiz, delete_question, delete_quiz, update_question, update_quiz_info,
    },
    database::pool::QuizBankPool,
};

pub fn create_route(pool: QuizBankPool) -> Router {
    Router::new()
        .route("/api/admin/quizzes", post(create_quiz))
        .route(
            "/api/admin/quizzes/{:id}",
            patch(update_quiz_info).delete(delete_quiz),
        )
        .route(
            "/api/admin/quizzes/{:quiz_id}/questions/",
            post(add_question),
        )
        .route(
            "/api/admin/quizzes/{:quiz_id}/questions/{:question_id}",
            put(update_question).delete(delete_question),
        )
        .with_state(pool)
}
