use std::sync::Arc;

use axum::{Router, routing::get};

use crate::features::{
    shared::app_state::AppState, submission_result::controller::get_me_submissions,
};

pub fn create_auth_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/users/me/submissions", get(get_me_submissions))
        .with_state(app_state)
}
