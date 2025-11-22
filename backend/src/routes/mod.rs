use axum::{Router, routing::get};

pub mod admin;
pub mod auth;
pub mod category;
pub mod quiz;
pub mod result;
pub mod user;

pub fn create_default_route() -> Router {
    Router::new().route("/", get(|| async { "Hello from csquizz!" }))
}
