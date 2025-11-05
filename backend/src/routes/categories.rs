use std::sync::Arc;

use axum::{routing::get, Router};
use tokio::sync::RwLock;

use crate::{app::AppState, controller::categories::get_categories};

pub fn create_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/categories", get(get_categories))
        .with_state(state)
}
