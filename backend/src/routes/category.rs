use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{app::AppState, controller::category};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route("/api/categories", get(category::get_page))
        .route("/api/categories/{:id}", get(category::find_by_id))
        .route("/api/categories/all", get(category::find_all))
        .with_state(state)
}

pub fn create_admin_route(state: AppState) -> Router {
    Router::new()
        .route("/api/admin/categories", post(category::create))
        .route(
            "/api/admin/categories/{:id}",
            delete(category::delete).patch(category::update),
        )
        .with_state(state)
}
