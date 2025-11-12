use axum::{routing::{get, patch}, Router};
use crate::interface::{
    controllers::user_controller,
    app_state::AppState,
    middleware::{auth_middleware::auth_middleware, admin_middleware::admin_middleware}, // Assuming these exist
};

pub fn create_user_routes() -> Router<AppState> {
    let protected_routes = Router::new()
        .route("/me", get(user_controller::get_me).patch(user_controller::update_me))
        .route_layer(axum::middleware::from_fn_with_state::<AppState, _>(auth_middleware));

    let admin_routes = Router::new()
        .route("/", get(user_controller::list_users))
        .route("/:id", patch(user_controller::update_user_role))
        .route_layer(axum::middleware::from_fn_with_state::<AppState, _>(admin_middleware)); // Apply admin middleware

    Router::new()
        .nest("/users", protected_routes)
        .nest("/admin/users", admin_routes)
}
