use axum::{routing::{post}, Router};
use crate::interface::{
    controllers::auth_controller,
    app_state::AppState, // Assuming AppState is defined in src/application/app_state.rs
};

pub fn create_auth_routes() -> Router<AppState> {
    Router::new()
        .route("/register", post(auth_controller::register))
        .route("/login", post(auth_controller::login))
        .route("/login/google", post(auth_controller::google_login))
}
