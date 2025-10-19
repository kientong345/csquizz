use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    database::pool::QuizBankPool,
    services::auth::{get_my_info, handle_login, handle_logout, handle_refresh, handle_register},
};

pub fn create_route(pool: QuizBankPool) -> Router {
    Router::new()
        .route("/api/auth/register", post(handle_register))
        .route("/api/auth/login", post(handle_login))
        .route("/api/auth/refresh", post(handle_refresh))
        .route("/api/auth/logout", post(handle_logout))
        .route("/api/auth/me", get(get_my_info))
        .with_state(pool)
}
