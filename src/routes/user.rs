use axum::{routing::get, Router};

use crate::{database::pool::QuizBankPool, services::results::get_results};

pub fn create_route(pool: QuizBankPool) -> Router {
    Router::new()
        .route("/api/user/results", get(get_results))
        .with_state(pool)
}
