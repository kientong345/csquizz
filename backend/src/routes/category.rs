use axum::{Router, routing::get};

use crate::{app::AppState, controller::category};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route("/api/categories", get(category::paginate))
        .with_state(state)
}
