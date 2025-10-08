use axum::Router;

use crate::database::pool::QuizBankPool;

use crate::routes;

pub async fn create_app(pool: QuizBankPool) -> Router {
    Router::new()
        .merge(routes::quizzes::create_route(pool.clone()))
        .merge(routes::user::create_route(pool.clone()))
        .merge(routes::categories::create_route(pool.clone()))
        .merge(routes::admin::create_route(pool))
}
