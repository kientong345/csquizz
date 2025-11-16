use std::sync::Arc;

use axum::{Router, routing::post};

use crate::features::{
    auth::controller::{google_login, login, register},
    shared::app_state::AppState,
};

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/google-login", post(google_login))
        .with_state(app_state)
}
