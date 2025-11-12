use std::sync::Arc;

use axum::{Router, middleware, routing::get};
use tokio::sync::RwLock;

use crate::{app::AppState, controller::user, middleware::auth::auth_middleware};

pub fn create_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/users/{:id}", get(user::get))
        .with_state(state)
}

pub fn create_protected_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .route("/api/users/me", get(user::get_me))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}
