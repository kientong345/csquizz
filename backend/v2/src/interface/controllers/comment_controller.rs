use axum::{extract::{State, Json, Path, Query}, response::IntoResponse, http::StatusCode};
use crate::{
    application::services::comment_service::CommentService,
    interface::dto::comment_dto::{CommentDto, CreateCommentDto, ListCommentsQuery},
    interface::dto::shared_dto::{PaginatedResponse, PaginationInfo},
};

// Placeholder for application state
#[derive(Clone)]
pub struct AppState {
    pub comment_service: CommentService,
    // Other services
}

/// Retrieves a paginated list of comments for a specific quiz.
pub async fn list_comments(
    State(app_state): State<AppState>,
    Query(query): Query<ListCommentsQuery>,
) -> impl IntoResponse {
    match app_state.comment_service.list_comments(query).await {
        Ok((comments, total_items, total_pages)) => {
            let comment_dtos: Vec<CommentDto> = comments; // Assuming service returns DTOs
            let pagination_info = PaginationInfo {
                current_page: query.page.unwrap_or(1),
                total_pages,
                total_items,
                limit: query.limit.unwrap_or(10),
            };
            (StatusCode::OK, Json(PaginatedResponse {
                data: comment_dtos,
                pagination: pagination_info,
            })).into_response()
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Creates a new comment on a quiz.
pub async fn create_comment(
    State(app_state): State<AppState>,
    current_user_id: i32, // From auth middleware
    Json(dto): Json<CreateCommentDto>,
) -> impl IntoResponse {
    match app_state.comment_service.create_comment(current_user_id, dto).await {
        Ok(comment) => (StatusCode::CREATED, Json(comment)).into_response(), // Assuming service returns DTO
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Deletes a comment.
pub async fn delete_comment(
    State(app_state): State<AppState>,
    Path(comment_id): Path<i32>,
    current_user_id: i32, // From auth middleware for authorization check
    // Admin role check by middleware
) -> impl IntoResponse {
    match app_state.comment_service.delete_comment(comment_id, current_user_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
