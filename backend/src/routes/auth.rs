use std::sync::Arc;

use axum::{routing::post, Router};
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
        .route("/api/register", post(handle_register))
        .route("/api/login", post(handle_login))
        .route("/api/oauth/google", post(handle_login_by_google))
        .route("/api/oauth/google/callback", post(handle_oauth_callback))
        .route("/api/logout", post(handle_logout))
        .route("/api/refresh", post(handle_refresh))
        .with_state(state)
}
