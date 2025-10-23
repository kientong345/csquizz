use axum::{routing::get, Router};

use crate::{
    controller::users::{get_my_info, get_my_result, get_user_info, get_user_results},
    database::pool::QuizBankPool,
};

pub fn create_route(pool: QuizBankPool) -> Router {
    Router::new()
        .route("/api/users/{:id}", get(get_user_info))
        .route("/api/users/{:id}/results", get(get_user_results))
        .with_state(pool)
}

pub fn create_protected_route(pool: QuizBankPool) -> Router {
    Router::new()
        .route("/api/users/me", get(get_my_info))
        .route("/api/users/me/results", get(get_my_result))
        .with_state(pool)
}
