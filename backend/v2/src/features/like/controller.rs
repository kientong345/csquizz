use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::features::shared::app_state::AppState;

pub async fn like_quiz(
    State(app_state): State<Arc<AppState>>,
    // Json(dto): Json<CreateLikeDto>,
) -> impl IntoResponse {
    // // Placeholder user ID
    // let current_user_id = 1;

    // match app_state
    //     .services
    //     .like_service
    //     .like_quiz(current_user_id, dto.target_id)
    //     .await
    // {
    //     Ok(_) => (StatusCode::CREATED, "Quiz liked").into_response(),
    //     Err(e) => e.into_response(),
    // }
}

pub async fn unlike_quiz(
    State(app_state): State<Arc<AppState>>,
    // Json(dto): Json<CreateLikeDto>,
) -> impl IntoResponse {
    // // Placeholder user ID
    // let current_user_id = 1;

    // match app_state
    //     .services
    //     .like_service
    //     .unlike_quiz(current_user_id, dto.target_id)
    //     .await
    // {
    //     Ok(_) => (StatusCode::NO_CONTENT).into_response(),
    //     Err(e) => e.into_response(),
    // }
}

pub async fn like_comment(
    State(app_state): State<Arc<AppState>>,
    // Json(dto): Json<CreateLikeDto>,
) -> impl IntoResponse {
    // // Placeholder user ID
    // let current_user_id = 1;

    // match app_state
    //     .services
    //     .like_service
    //     .like_comment(current_user_id, dto.target_id)
    //     .await
    // {
    //     Ok(_) => (StatusCode::CREATED, "Comment liked").into_response(),
    //     Err(e) => e.into_response(),
    // }
}

pub async fn unlike_comment(
    State(app_state): State<Arc<AppState>>,
    // Json(dto): Json<CreateLikeDto>,
) -> impl IntoResponse {
    // // Placeholder user ID
    // let current_user_id = 1;

    // match app_state
    //     .services
    //     .like_service
    //     .unlike_comment(current_user_id, dto.target_id)
    //     .await
    // {
    //     Ok(_) => (StatusCode::NO_CONTENT).into_response(),
    //     Err(e) => e.into_response(),
    // }
}
