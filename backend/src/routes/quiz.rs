use axum::{Router, routing::get};

use crate::{app::AppState, controller::quiz};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route("/api/quizzes", get(quiz::paginate))
        .route("/api/quizzes/{:id}", get(quiz::get))
        .with_state(state)
}
