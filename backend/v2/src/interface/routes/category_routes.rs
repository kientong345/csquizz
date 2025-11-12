use axum::{routing::{get, post, put, delete}, Router};
use crate::interface::{
    controllers::category_controller,
    app_state::AppState,
    middleware::admin_middleware::admin_middleware, // Assuming this exists
};

pub fn create_category_routes() -> Router<AppState> {
    let admin_routes = Router::new()
        .route("/", post(category_controller::create_category))
        .route("/:id", put(category_controller::update_category).delete(category_controller::delete_category))
        .route_layer(axum::middleware::from_fn_with_state::<AppState, _>(admin_middleware));

    Router::new()
        .route("/", get(category_controller::list_categories))
        .nest("/admin/categories", admin_routes)
}
