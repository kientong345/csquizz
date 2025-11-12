use axum::{routing::{post, delete}, Router};
use crate::interface::{
    controllers::like_controller,
    app_state::AppState,
    middleware::auth_middleware::auth_middleware, // Assuming this exists
};

pub fn create_like_routes() -> Router<AppState> {
    Router::new()
        .route("/quiz", post(like_controller::like_quiz).delete(like_controller::unlike_quiz))
        .route("/comment", post(like_controller::like_comment).delete(like_controller::unlike_comment))
        .route_layer(axum::middleware::from_fn_with_state::<AppState, _>(auth_middleware))
}
