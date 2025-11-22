use axum::{
    Router, middleware,
    routing::{delete, get, patch},
};

use crate::{app::AppState, controller::user, middleware::auth::auth_middleware};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route("/api/users", get(user::get_page))
        .route("/api/users/{:id}", get(user::find_by_id))
        .with_state(state)
}

pub fn create_auth_route(state: AppState) -> Router {
    Router::new()
        .route("/api/users/me", get(user::get_me).patch(user::update_me))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}

pub fn create_admin_route(state: AppState) -> Router {
    Router::new()
        .route("/api/admin/users/{:id}", delete(user::delete))
        .route("/api/admin/users", patch(user::update))
        .with_state(state)
}
