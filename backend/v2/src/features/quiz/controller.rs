use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::{
    domain::quiz::model::UpdateQuizParams,
    features::{
        quiz::dto::{CreateQuizParamsDto, QuizQueryDto, UpdateQuizParamsDto},
        shared::app_state::AppState,
    },
};

pub async fn get_quizzes_page(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<QuizQueryDto>,
) -> impl IntoResponse {
    // let page = query.page.unwrap_or(1);
    // let limit = query.limit.unwrap_or(10);

    // match app_state
    //     .services
    //     .quiz_service
    //     .list_quizzes(
    //         query.category_id,
    //         query.difficulty,
    //         query.search_term,
    //         page,
    //         limit,
    //         query.sort_by,
    //         query.order,
    //     )
    //     .await
    // {
    //     Ok(paginated_result) => (StatusCode::OK, Json(paginated_result)).into_response(),
    //     Err(e) => e.into_response(),
    // }
}

pub async fn get_quiz(
    State(app_state): State<Arc<AppState>>,
    Path(quiz_id): Path<i32>,
) -> impl IntoResponse {
    // match app_state.services.quiz_service.get_quiz(quiz_id).await {
    //     Ok(quiz) => (StatusCode::OK, Json(quiz)).into_response(),
    //     Err(e) => e.into_response(),
    // }
}

pub async fn create_quiz(
    State(app_state): State<Arc<AppState>>,
    // In a real app, this would be extracted from the JWT token by an auth middleware
    // let current_user: CurrentUser = ...;
    Json(dto): Json<CreateQuizParamsDto>,
) -> impl IntoResponse {
    // // Placeholder for authenticated user ID
    // let current_user_id = 1; // Assume admin user

    // match app_state
    //     .services
    //     .quiz_service
    //     .create_quiz(
    //         dto.title,
    //         dto.description,
    //         &dto.difficulty,
    //         dto.category_id,
    //         current_user_id,
    //     )
    //     .await
    // {
    //     Ok(quiz_detail) => (StatusCode::CREATED, Json(quiz_detail)).into_response(),
    //     Err(e) => e.into_response(),
    // }
}

pub async fn update_quiz(
    State(app_state): State<Arc<AppState>>,
    Path(quiz_id): Path<i32>,
    Json(dto): Json<UpdateQuizParamsDto>,
) -> impl IntoResponse {
    // match app_state
    //     .services
    //     .quiz_service
    //     .update_quiz(
    //         quiz_id,
    //         dto.title,
    //         dto.description,
    //         dto.difficulty,
    //         dto.category_id,
    //     )
    //     .await
    // {
    //     Ok(quiz_detail) => (StatusCode::OK, Json(quiz_detail)).into_response(),
    //     Err(e) => e.into_response(),
    // }
}

pub async fn delete_quiz(
    State(app_state): State<Arc<AppState>>,
    Path(quiz_id): Path<i32>,
) -> impl IntoResponse {
    // match app_state.services.quiz_service.delete_quiz(quiz_id).await {
    //     Ok(_) => (StatusCode::NO_CONTENT).into_response(),
    //     Err(e) => e.into_response(),
    // }
}

// // Helper to convert ServiceError to a response
// impl IntoResponse for ServiceError {
//     fn into_response(self) -> axum::response::Response {
//         let (status, error_message) = match self {
//             ServiceError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
//             ServiceError::NotFound => (
//                 StatusCode::NOT_FOUND,
//                 "The requested resource was not found".to_string(),
//             ),
//             ServiceError::Forbidden => (
//                 StatusCode::FORBIDDEN,
//                 "You do not have permission to perform this action".to_string(),
//             ),
//             ServiceError::Conflict(msg) => (StatusCode::CONFLICT, msg),
//             ServiceError::Internal => (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 "An internal server error occurred".to_string(),
//             ),
//         };
//         (status, Json(serde_json::json!({ "error": error_message }))).into_response()
//     }
// }
