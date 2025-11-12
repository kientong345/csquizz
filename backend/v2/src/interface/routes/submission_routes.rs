use axum::{routing::{post, get}, Router};
use crate::interface::{
    controllers::submission_controller,
    app_state::AppState,
    middleware::auth_middleware::auth_middleware, // Assuming this exists
};

pub fn create_submission_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(submission_controller::submit_quiz))
        .route("/:id", get(submission_controller::get_submission_result))
        .route("/me", get(submission_controller::list_my_submissions))
        .route_layer(axum::middleware::from_fn_with_state::<AppState, _>(auth_middleware))
}
