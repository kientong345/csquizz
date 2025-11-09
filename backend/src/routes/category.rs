use std::sync::Arc;

use axum::{Router, routing::get};
use tokio::sync::RwLock;

use crate::{app::AppState, controller::category};

pub fn create_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/categories", get(category::paginate))
        .with_state(state)
}
