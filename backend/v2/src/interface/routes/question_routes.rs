use axum::{routing::{get, post, put, delete}, Router};
use crate::interface::{
    controllers::question_controller,
    app_state::AppState,
    middleware::admin_middleware::admin_middleware, // Assuming this exists
};

pub fn create_question_routes() -> Router<AppState> {
    let admin_routes = Router::new()
        .route("/", post(question_controller::create_question))
        .route("/:id", put(question_controller::update_question).delete(question_controller::delete_question))
        .route_layer(axum::middleware::from_fn_with_state::<AppState, _>(admin_middleware));

    Router::new()
        .route("/", get(question_controller::list_questions))
        .route("/:id", get(question_controller::get_question))
        .nest("/admin/questions", admin_routes)
}
