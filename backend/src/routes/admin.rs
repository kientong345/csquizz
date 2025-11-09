use std::sync::Arc;

use axum::Router;
use tokio::sync::RwLock;

use crate::app::AppState;

pub fn create_protected_route(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        // .route("/api/admin/quizzes", post(quiz))
        // .route(
        //     "/api/admin/quizzes/{:id}",
        //     patch(update_quiz_info).delete(delete_quiz),
        // )
        // .route(
        //     "/api/admin/quizzes/{:quiz_id}/questions/",
        //     post(add_question),
        // )
        // .route(
        //     "/api/admin/quizzes/{:quiz_id}/questions/{:question_id}",
        //     put(update_question).delete(delete_question),
        // )
        .with_state(state)
}
