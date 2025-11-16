use std::sync::Arc;

use axum::{
    Router,
    routing::{delete, post},
};

use crate::features::{
    like::controller::{like_comment, like_quiz, unlike_comment, unlike_quiz},
    shared::app_state::AppState,
};

pub fn create_routes(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/quiz/{:quiz_id}/like", post(like_quiz))
        .route("/api/quiz/{:quiz_id}/unlike", delete(unlike_quiz))
        .route("/api/comment/{:comment_id}/like", post(like_comment))
        .route("/api/comment/{:comment_id}/unlike", delete(unlike_comment))
        .with_state(app_state)
}
