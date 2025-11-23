use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{
    app::AppState,
    controller::quiz::{
        add_question, comment_quiz, create_quiz_with_questions, delete_question, delete_quiz,
        get_quiz_with_comments, get_quiz_with_questions, get_quizzes_page, like_quiz,
        update_question, update_quiz_metadata,
    },
};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route("/api/quizzes", get(get_quizzes_page))
        .route("/api/quizzes/{:id}/questions", get(get_quiz_with_questions))
        .route("/api/quizzes/{:id}/comments", get(get_quiz_with_comments))
        .with_state(state)
}

pub fn create_auth_route(state: AppState) -> Router {
    Router::new()
        .route("/api/quizzes", post(create_quiz_with_questions))
        .route("/api/quizzes/{:id}/like", post(like_quiz))
        .route("/api/quizzes/{:id}/comment", post(comment_quiz))
        .with_state(state)
}

pub fn create_owner_route(state: AppState) -> Router {
    Router::new()
        .route("/api/quizzes/{:id}/questions", post(add_question))
        .route(
            "/api/quizzes/{:id}/questions/{:question_id}",
            delete(delete_question).patch(update_question),
        )
        .route(
            "/api/quizzes/{:id}",
            delete(delete_quiz).patch(update_quiz_metadata),
        )
        .with_state(state)
}
