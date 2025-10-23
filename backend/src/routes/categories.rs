use axum::{routing::get, Router};

use crate::{controller::categories::get_categories, database::pool::QuizBankPool};

pub fn create_route(pool: QuizBankPool) -> Router {
    Router::new()
        .route("/api/categories", get(get_categories))
        .with_state(pool)
}
