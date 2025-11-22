use axum::{Router, routing::get};

use crate::{app::AppState, controller::question};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route("/api/questions", get(question::paginate))
        .with_state(state)
}
