use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post, put},
};

use crate::features::{
    category::controller::{create_category, delete_category, get_category_page, update_category},
    shared::app_state::AppState,
};

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/categories", get(get_category_page))
        .with_state(app_state)
}

pub fn create_admin_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/categories", post(create_category))
        .route(
            "/api/categories/{:id}",
            put(update_category).delete(delete_category),
        )
        .with_state(app_state)
}
