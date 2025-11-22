use axum::{Router, routing::put};

use crate::{app::AppState, controller::admin};

pub fn create_admin_route(state: AppState) -> Router {
    Router::new()
        .route("/api/admin/grant", put(admin::grant_admin_permission))
        .with_state(state)
}
