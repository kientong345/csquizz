use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::sync::RwLock;

use crate::{app::AppState, controller::result};

pub fn create_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/results/{:user_id}", get(result::paginate))
        .with_state(state)
}

pub fn create_protected_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/results/me", get(result::paginate_me))
        .with_state(state)
}
