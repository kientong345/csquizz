use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::features::{
    comment::dto::{CommentQueryDto, CreateCommentDto},
    shared::app_state::AppState,
};

pub async fn get_comments_page(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<CommentQueryDto>,
) -> impl IntoResponse {
    // let page = query.page.unwrap_or(1);
    // let limit = query.limit.unwrap_or(10);

    // match app_state
    //     .services
    //     .comment_service
    //     .list_comments(query.quiz_id, page, limit)
    //     .await
    // {
    //     Ok(paginated_comments) => (StatusCode::OK, Json(paginated_comments)).into_response(),
    //     Err(e) => e.into_response(),
    // }
}

pub async fn create_comment(
    State(app_state): State<Arc<AppState>>,
    // Placeholder for authenticated user
    Json(dto): Json<CreateCommentDto>,
) -> impl IntoResponse {
    // // Placeholder user ID
    // let current_user_id = 1;

    // match app_state
    //     .services
    //     .comment_service
    //     .create_comment(dto.quiz_id, current_user_id, dto.content)
    //     .await
    // {
    //     Ok(comment) => (StatusCode::CREATED, Json(comment)).into_response(),
    //     Err(e) => e.into_response(),
    // }
}

pub async fn update_comment(
    State(app_state): State<Arc<AppState>>,
    Path(comment_id): Path<i32>,
    // Placeholder for authenticated user
) -> impl IntoResponse {
    // // Placeholder user ID
    // let current_user_id = 1;

    // match app_state
    //     .services
    //     .comment_service
    //     .delete_comment(comment_id, current_user_id)
    //     .await
    // {
    //     Ok(_) => (StatusCode::NO_CONTENT).into_response(),
    //     Err(e) => e.into_response(),
    // }
}

pub async fn delete_comment(
    State(app_state): State<Arc<AppState>>,
    Path(comment_id): Path<i32>,
    // Placeholder for authenticated user
) -> impl IntoResponse {
    // // Placeholder user ID
    // let current_user_id = 1;

    // match app_state
    //     .services
    //     .comment_service
    //     .delete_comment(comment_id, current_user_id)
    //     .await
    // {
    //     Ok(_) => (StatusCode::NO_CONTENT).into_response(),
    //     Err(e) => e.into_response(),
    // }
}
