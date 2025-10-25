use axum::{middleware, Router};

use crate::database::pool::QuizBankPool;

use crate::middleware::auth::auth_middleware;
use crate::routes;

pub async fn create_app(pool: QuizBankPool) -> Router {
    Router::new()
        .merge(routes::user::create_protected_route(pool.clone()))
        .layer(middleware::from_fn(auth_middleware))
        .merge(routes::quizzes::create_route(pool.clone()))
        .merge(routes::user::create_route(pool.clone()))
        .merge(routes::categories::create_route(pool.clone()))
        .merge(routes::admin::create_route(pool.clone()))
        .merge(routes::auth::create_route(pool))
}
