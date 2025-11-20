use axum::Router;
use csquizz::{
    api::routes,
    config::Configuration,
    features::{
        auth::service::AuthService, category::service::CategoryService,
        comment::service::CommentService, like::service::LikeService, quiz::service::QuizService,
        shared::app_state::AppState, user::service::UserService,
    },
    infrastructure::{
        database::postgres_context::DatabasePool,
        repositories::{
            category::SqlxCategoryRepository, comment::SqlxCommentRepository, like::SqlxLikeRepository,
            question::SqlxQuestionRepository, quiz::SqlxQuizRepository, user::SqlxUserRepository,
        },
    },
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Load configuration
    let config = Configuration::get();

    // Start server
    #[cfg(feature = "local")]
    let address = SocketAddr::from(([127, 0, 0, 1], config.app_config.port));

    #[cfg(not(feature = "local"))]
    let address = SocketAddr::from(([0, 0, 0, 0], config.app_config.port));

    let listener = TcpListener::bind(address)
        .await
        .expect("cannot bind address");

    // Create application state
    let app_state = Arc::new(build_app_state(config).await);

    // Create app
    let app = create_app(app_state).await;

    // Serve app
    axum::serve(listener, app)
        .await
        .expect("cannot serving app");
}

async fn build_app_state(config: Configuration) -> AppState {
    let db_pool = Arc::new(DatabasePool::init(&config.db_config).await);

    let category_repository = Arc::new(SqlxCategoryRepository::init(db_pool.clone()));
    let user_repository = Arc::new(SqlxUserRepository::init(db_pool.clone()));
    let quiz_repository = Arc::new(SqlxQuizRepository::init(db_pool.clone()));
    let question_repository = Arc::new(SqlxQuestionRepository::init(db_pool.clone()));
    let comment_repository = Arc::new(SqlxCommentRepository::init(db_pool.clone()));
    let like_repository = Arc::new(SqlxLikeRepository::init(db_pool.clone()));

    AppState {
        auth_service: AuthService::build_from(user_repository.clone(), config.auth_config),
        category_service: CategoryService::build_from(category_repository),
        comment_service: CommentService::build_from(comment_repository),
        like_service: LikeService::build_from(like_repository),
        quiz_service: QuizService::build_from(quiz_repository),
        user_service: UserService::build_from(user_repository),
    }
}

async fn create_app(app_state: Arc<AppState>) -> Router {
    Router::new()
        .merge(routes::auth::create_routes(app_state.clone()))
        .merge(routes::category::create_routes(app_state.clone()))
        .merge(routes::quiz::create_routes(app_state.clone()))
        .merge(routes::like::create_routes(app_state.clone()))
        .merge(routes::submission::create_auth_route(app_state.clone()))
        .merge(routes::comment::create_routes(app_state.clone()))
        .merge(routes::user::create_routes(app_state.clone()))
}
