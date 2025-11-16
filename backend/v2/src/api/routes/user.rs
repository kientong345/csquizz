use std::sync::Arc;

use crate::features::{
    shared::app_state::AppState,
    user::controller::{get_me, get_user, get_users_page, update_me, update_user_role},
};
use axum::{
    Router,
    routing::{get, patch},
};

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/users", get(get_users_page))
        .route("/api/users/{:id}", get(get_user))
        .with_state(app_state)
}

pub fn create_auth_route(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/users/me", get(get_me).patch(update_me))
        .with_state(app_state)
}

pub fn create_admin_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/users/{:id}", patch(update_user_role))
        .with_state(app_state)
}
