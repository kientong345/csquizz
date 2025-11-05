use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::sync::RwLock;

use crate::{
    app::AppState,
    controller::quizzes::{get_questions, get_quiz_by_id, get_quizzes, submit_quiz},
};

pub fn create_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/quizzes", get(get_quizzes))
        .route("/api/quizzes/{:id}", get(get_quiz_by_id))
        .route("/api/quizzes/{:id}/submit", post(submit_quiz))
        .route("/api/questions", get(get_questions))
        .with_state(state)
}
