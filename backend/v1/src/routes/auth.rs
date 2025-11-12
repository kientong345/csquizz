use std::sync::Arc;

use axum::{Router, routing::post};
use tokio::sync::RwLock;

use crate::{
    app::AppState,
    controller::auth::{
        handle_login, handle_login_by_google, handle_logout, handle_oauth_callback, handle_refresh,
        handle_register,
    },
};

pub fn create_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/auth/register", post(handle_register))
        .route("/api/auth/login", post(handle_login))
        .route("/api/auth/oauth/google", post(handle_login_by_google))
        .route("/api/auth/oauth/google/callback", post(handle_oauth_callback))
        .route("/api/auth/logout", post(handle_logout))
        .route("/api/auth/refresh", post(handle_refresh))
        .with_state(state)
}
