use axum::{routing::get, Router};

use crate::{database::pool::QuizBankPool, services::categories::get_categories};

pub fn create_route(pool: QuizBankPool) -> Router {
    Router::new()
        .route("/api/categories", get(get_categories))
        .with_state(pool)
}
