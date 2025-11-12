use axum::{routing::{get, post, delete}, Router};
use crate::interface::{
    controllers::comment_controller,
    app_state::AppState,
    middleware::auth_middleware::auth_middleware, // Assuming this exists
};

pub fn create_comment_routes() -> Router<AppState> {
    let protected_routes = Router::new()
        .route("/", post(comment_controller::create_comment))
        .route("/:id", delete(comment_controller::delete_comment))
        .route_layer(axum::middleware::from_fn_with_state::<AppState, _>(auth_middleware));

    Router::new()
        .route("/", get(comment_controller::list_comments))
        .nest("/", protected_routes)
}
