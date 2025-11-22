use axum::{Router, routing::post};

use crate::{app::AppState, controller::submission};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route("/api/submissions", post(submission::unauthorized_submit))
        .with_state(state)
}

pub fn create_protected_route(state: AppState) -> Router {
    Router::new()
        .route("/api/submissions/me", post(submission::submit))
        .with_state(state)
}
