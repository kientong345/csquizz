use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::sync::RwLock;

use crate::{app::AppState, controller::quiz};

pub fn create_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/quizzes", get(quiz::paginate))
        .route("/api/quizzes/{:id}", get(quiz::get))
        .with_state(state)
}
