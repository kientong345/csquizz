use std::sync::Arc;

use axum::Router;

use crate::config::Configuration;
use crate::database::non_persistent::SecondaryDatabase;
use crate::database::persistent::PrimaryDatabase;

use crate::routes;

use crate::services::auth::AuthService;
use crate::services::category::CategoryService;
use crate::services::quiz::QuizService;
use crate::services::user::UserService;

#[derive(Clone)]
pub struct AppState {
    pub primary_db: PrimaryDatabase,
    pub secondary_db: Option<SecondaryDatabase>,
    pub config: Arc<Configuration>,
    pub quiz_service: QuizService,
    pub category_service: CategoryService,
    pub auth_service: AuthService,
    pub user_service: UserService,
}

pub async fn create_app(state: AppState) -> Router {
    Router::new()
        // Admin routes
        .merge(routes::admin::create_admin_route(state.clone()))
        // auth routes
        .merge(routes::auth::create_route(state.clone()))
        // user routes
        .merge(routes::user::create_route(state.clone()))
        .merge(routes::user::create_auth_route(state.clone()))
        .merge(routes::user::create_admin_route(state.clone()))
        // category routes
        .merge(routes::category::create_route(state.clone()))
        .merge(routes::category::create_admin_route(state.clone()))
        // quiz routes
        .merge(routes::quiz::create_route(state.clone()))
        .merge(routes::quiz::create_auth_route(state.clone()))
        .merge(routes::quiz::create_owner_route(state.clone()))
        // default routes
        .merge(routes::create_default_route())
}
