use std::sync::Arc;

use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::features::shared::app_state::AppState;

pub async fn get_me_submissions(
    State(app_state): State<Arc<AppState>>,
    // current_user_id: i32, // From auth middleware
    // Query(query): Query<SubmissionQueryDto>,
) -> impl IntoResponse {
    // match app_state
    //     .submission_service
    //     .list_user_submissions(current_user_id, query)
    //     .await
    // {
    //     Ok((submissions, total_items, total_pages)) => {
    //         let submission_dtos: Vec<SubmissionHistoryItemDto> = submissions; // Assuming service returns DTOs
    //         let pagination_info = PaginationInfo {
    //             current_page: query.page.unwrap_or(1),
    //             total_pages,
    //             total_items,
    //             limit: query.limit.unwrap_or(10),
    //         };
    //         (
    //             StatusCode::OK,
    //             Json(PaginatedResponse {
    //                 data: submission_dtos,
    //                 pagination: pagination_info,
    //             }),
    //         )
    //             .into_response()
    //     }
    //     Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    // }
}

// /// Handles quiz submission.
// pub async fn submit_quiz(
//     State(app_state): State<AppState>,
//     current_user_id: i32, // From auth middleware
//     Json(dto): Json<SubmitQuizDto>,
// ) -> impl IntoResponse {
//     match app_state
//         .submission_service
//         .submit_quiz(current_user_id, dto)
//         .await
//     {
//         Ok(result) => (StatusCode::CREATED, Json(result)).into_response(),
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
// }

// /// Retrieves details of a specific submission result.
// pub async fn get_submission_result(
//     State(app_state): State<AppState>,
//     Path(submission_id): Path<i32>,
//     current_user_id: i32, // From auth middleware for authorization check
// ) -> impl IntoResponse {
//     match app_state
//         .submission_service
//         .get_submission_result(submission_id, current_user_id)
//         .await
//     {
//         Ok(Some(result)) => (StatusCode::OK, Json(result)).into_response(),
//         Ok(None) => StatusCode::NOT_FOUND.into_response(),
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
// }

// /// Retrieves a paginated list of the current user's submission history.
// pub async fn list_my_submissions(
//     State(app_state): State<AppState>,
//     current_user_id: i32, // From auth middleware
//     Query(query): Query<ListSubmissionsQuery>,
// ) -> impl IntoResponse {
//     match app_state
//         .submission_service
//         .list_user_submissions(current_user_id, query)
//         .await
//     {
//         Ok((submissions, total_items, total_pages)) => {
//             let submission_dtos: Vec<SubmissionHistoryItemDto> = submissions; // Assuming service returns DTOs
//             let pagination_info = PaginationInfo {
//                 current_page: query.page.unwrap_or(1),
//                 total_pages,
//                 total_items,
//                 limit: query.limit.unwrap_or(10),
//             };
//             (
//                 StatusCode::OK,
//                 Json(PaginatedResponse {
//                     data: submission_dtos,
//                     pagination: pagination_info,
//                 }),
//             )
//                 .into_response()
//         }
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
// }
