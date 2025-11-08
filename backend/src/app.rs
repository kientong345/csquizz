use std::sync::Arc;

use axum::Router;
use tokio::sync::RwLock;

use crate::config::Configuration;
use crate::database::pool::QuizBankPool;

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: QuizBankPool,
    pub config: Configuration,
}

pub async fn create_app(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .merge(routes::user::create_protected_route(state.clone()))
        .merge(routes::quizzes::create_route(state.clone()))
        .merge(routes::user::create_route(state.clone()))
        .merge(routes::categories::create_route(state.clone()))
        .merge(routes::admin::create_route(state.clone()))
        .merge(routes::auth::create_route(state))
        .merge(routes::create_default_route())
}
