use axum::{
    Router, middleware,
    routing::{delete, get},
};

use crate::{
    app::AppState,
    controller::user::{
        delete_user, find_user_by_id, get_me, get_submissions_me, get_users_page, update_me,
        update_user,
    },
    middleware::auth::auth_middleware,
};

pub fn create_route(state: AppState) -> Router {
    Router::new()
        .route("/api/users", get(get_users_page))
        .route("/api/users/{:id}", get(find_user_by_id))
        .with_state(state)
}

pub fn create_auth_route(state: AppState) -> Router {
    Router::new()
        .route("/api/users/me", get(get_me).patch(update_me))
        .route("/api/users/me/submissions", get(get_submissions_me))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state)
}

pub fn create_admin_route(state: AppState) -> Router {
    Router::new()
        .route(
            "/api/admin/users/{:id}",
            delete(delete_user).patch(update_user),
        )
        .with_state(state)
}
