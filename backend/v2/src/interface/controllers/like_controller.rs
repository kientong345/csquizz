use axum::{extract::{State, Json}, response::IntoResponse, http::StatusCode};
use crate::{
    application::services::like_service::LikeService,
    interface::dto::like_dto::{QuizLikeDto, CommentLikeDto},
};

// Placeholder for application state
#[derive(Clone)]
pub struct AppState {
    pub like_service: LikeService,
    // Other services
}

/// Handles liking a quiz.
pub async fn like_quiz(
    State(app_state): State<AppState>,
    current_user_id: i32, // From auth middleware
    Json(dto): Json<QuizLikeDto>,
) -> impl IntoResponse {
    match app_state.like_service.like_quiz(current_user_id, dto.quiz_id).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Handles unliking a quiz.
pub async fn unlike_quiz(
    State(app_state): State<AppState>,
    current_user_id: i32, // From auth middleware
    Json(dto): Json<QuizLikeDto>,
) -> impl IntoResponse {
    match app_state.like_service.unlike_quiz(current_user_id, dto.quiz_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Handles liking a comment.
pub async fn like_comment(
    State(app_state): State<AppState>,
    current_user_id: i32, // From auth middleware
    Json(dto): Json<CommentLikeDto>,
) -> impl IntoResponse {
    match app_state.like_service.like_comment(current_user_id, dto.comment_id).await {
        Ok(_) => StatusCode::CREATED.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Handles unliking a comment.
pub async fn unlike_comment(
    State(app_state): State<AppState>,
    current_user_id: i32, // From auth middleware
    Json(dto): Json<CommentLikeDto>,
) -> impl IntoResponse {
    match app_state.like_service.unlike_comment(current_user_id, dto.comment_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
