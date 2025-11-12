use axum::{routing::{get, post, put, delete}, Router};
use crate::interface::{
    controllers::quiz_controller,
    app_state::AppState,
    middleware::admin_middleware::admin_middleware, // Assuming this exists
};

pub fn create_quiz_routes() -> Router<AppState> {
    let admin_routes = Router::new()
        .route("/", post(quiz_controller::create_quiz))
        .route("/:id", put(quiz_controller::update_quiz).delete(quiz_controller::delete_quiz))
        .route_layer(axum::middleware::from_fn_with_state::<AppState, _>(admin_middleware));

    Router::new()
        .route("/", get(quiz_controller::list_quizzes))
        .route("/:id", get(quiz_controller::get_quiz))
        .nest("/admin/quizzes", admin_routes)
}
