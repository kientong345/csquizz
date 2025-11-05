use std::sync::Arc;

use axum::{middleware, routing::get, Router};
use tokio::sync::RwLock;

use crate::{
    app::AppState,
    controller::users::{get_my_info, get_my_result, get_user_by_id, get_user_results},
    middleware::auth::auth_middleware,
};

pub fn create_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/users/{:id}", get(get_user_by_id))
        .route("/api/users/{:id}/results", get(get_user_results))
        .with_state(state)
}

pub fn create_protected_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/users/me", get(get_my_info))
        .route("/api/users/me/results", get(get_my_result))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
