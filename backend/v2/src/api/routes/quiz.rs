use std::sync::Arc;

use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::features::{
    quiz::controller::{create_quiz, delete_quiz, get_quiz, get_quizzes_page, update_quiz},
    shared::app_state::AppState,
};

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/quizzes", get(get_quizzes_page))
        .route("/api/quizzes/{:id}", get(get_quiz))
        .with_state(app_state)
}

pub fn create_auth_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/quizzes/", post(create_quiz))
        .route("/api/quizzes/{:id}", patch(update_quiz).delete(delete_quiz))
        .with_state(app_state)
}
