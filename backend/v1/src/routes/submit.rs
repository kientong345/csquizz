use std::sync::Arc;

use axum::{Router, routing::post};
use tokio::sync::RwLock;

use crate::{app::AppState, controller::submit};

pub fn create_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/submissions", post(submit::unauthorized_submit))
        .with_state(state)
}

pub fn create_protected_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/submissions/me", post(submit::submit))
        .with_state(state)
}
