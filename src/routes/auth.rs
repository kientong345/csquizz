use axum::{routing::post, Router};

use crate::{
    database::pool::QuizBankPool,
    services::auth::{handle_login, handle_logout, handle_refresh, handle_register},
};

pub fn create_route(pool: QuizBankPool) -> Router {
    Router::new()
        .route("/api/auth/register", post(handle_register))
        .route("/api/auth/login", post(handle_login))
        .route("/api/auth/logout", post(handle_logout))
        .route("/api/auth/refresh", post(handle_refresh))
        .with_state(pool)
}
