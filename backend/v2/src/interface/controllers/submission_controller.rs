use axum::{extract::{State, Json, Path, Query}, response::IntoResponse, http::StatusCode};
use crate::{
    application::services::submission_service::SubmissionService,
    interface::dto::submission_dto::{SubmitQuizDto, QuizResultDto, SubmissionHistoryItemDto, ListSubmissionsQuery},
    interface::dto::shared_dto::{PaginatedResponse, PaginationInfo},
};

// Placeholder for application state
#[derive(Clone)]
pub struct AppState {
    pub submission_service: SubmissionService,
    // Other services
}

/// Handles quiz submission.
pub async fn submit_quiz(
    State(app_state): State<AppState>,
    current_user_id: i32, // From auth middleware
    Json(dto): Json<SubmitQuizDto>,
) -> impl IntoResponse {
    match app_state.submission_service.submit_quiz(current_user_id, dto).await {
        Ok(result) => (StatusCode::CREATED, Json(result)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Retrieves details of a specific submission result.
pub async fn get_submission_result(
    State(app_state): State<AppState>,
    Path(submission_id): Path<i32>,
    current_user_id: i32, // From auth middleware for authorization check
) -> impl IntoResponse {
    match app_state.submission_service.get_submission_result(submission_id, current_user_id).await {
        Ok(Some(result)) => (StatusCode::OK, Json(result)).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Retrieves a paginated list of the current user's submission history.
pub async fn list_my_submissions(
    State(app_state): State<AppState>,
    current_user_id: i32, // From auth middleware
    Query(query): Query<ListSubmissionsQuery>,
) -> impl IntoResponse {
    match app_state.submission_service.list_user_submissions(current_user_id, query).await {
        Ok((submissions, total_items, total_pages)) => {
            let submission_dtos: Vec<SubmissionHistoryItemDto> = submissions; // Assuming service returns DTOs
            let pagination_info = PaginationInfo {
                current_page: query.page.unwrap_or(1),
                total_pages,
                total_items,
                limit: query.limit.unwrap_or(10),
            };
            (StatusCode::OK, Json(PaginatedResponse {
                data: submission_dtos,
                pagination: pagination_info,
            })).into_response()
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
