use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post, put},
};

use crate::features::{
    comment::controller::{create_comment, delete_comment, get_comments_page, update_comment},
    shared::app_state::AppState,
};

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/quiz/{:quiz_id}/comments", get(get_comments_page))
        .with_state(app_state)
}

pub fn create_auth_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/quiz/{:quiz_id}/comments", post(create_comment))
        .route(
            "/api/quiz/{:quiz_id}/comments/{:comment_id}",
            put(update_comment).delete(delete_comment),
        )
        .with_state(app_state)
}
