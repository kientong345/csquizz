use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::sync::RwLock;

use crate::{app::AppState, controller::question};

pub fn create_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/questions", get(question::paginate))
        .with_state(state)
}
