use axum::{extract::{State, Json, Path, Query}, response::IntoResponse, http::StatusCode};
use crate::{
    application::services::quiz_service::QuizService,
    interface::dto::quiz_dto::{QuizDto, CreateQuizDto, UpdateQuizDto, ListQuizzesQuery},
    interface::dto::shared_dto::{PaginatedResponse, PaginationInfo},
};

// Placeholder for application state
#[derive(Clone)]
pub struct AppState {
    pub quiz_service: QuizService,
    // Other services
}

/// Retrieves a paginated list of quizzes.
pub async fn list_quizzes(
    State(app_state): State<AppState>,
    Query(query): Query<ListQuizzesQuery>,
) -> impl IntoResponse {
    match app_state.quiz_service.list_quizzes(query).await {
        Ok((quizzes, total_items, total_pages)) => {
            let quiz_dtos: Vec<QuizDto> = quizzes.into_iter().map(QuizDto::from).collect();
            let pagination_info = PaginationInfo {
                current_page: query.page.unwrap_or(1),
                total_pages,
                total_items,
                limit: query.limit.unwrap_or(10),
            };
            (StatusCode::OK, Json(PaginatedResponse {
                data: quiz_dtos,
                pagination: pagination_info,
            })).into_response()
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Admin: Creates a new quiz.
pub async fn create_quiz(
    State(app_state): State<AppState>,
    Json(dto): Json<CreateQuizDto>,
    current_user_id: i32, // From auth middleware
    // Admin role check by middleware
) -> impl IntoResponse {
    match app_state.quiz_service.create_quiz(dto, current_user_id).await {
        Ok(quiz) => (StatusCode::CREATED, Json(QuizDto::from(quiz))).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Retrieves details of a specific quiz.
pub async fn get_quiz(
    State(app_state): State<AppState>,
    Path(quiz_id): Path<i32>,
) -> impl IntoResponse {
    match app_state.quiz_service.get_quiz_by_id(quiz_id).await {
        Ok(Some(quiz)) => (StatusCode::OK, Json(QuizDto::from(quiz))).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Admin: Updates an existing quiz.
pub async fn update_quiz(
    State(app_state): State<AppState>,
    Path(quiz_id): Path<i32>,
    Json(dto): Json<UpdateQuizDto>,
    // Admin role check by middleware
) -> impl IntoResponse {
    match app_state.quiz_service.update_quiz(quiz_id, dto).await {
        Ok(quiz) => (StatusCode::OK, Json(QuizDto::from(quiz))).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Admin: Deletes a quiz.
pub async fn delete_quiz(
    State(app_state): State<AppState>,
    Path(quiz_id): Path<i32>,
    // Admin role check by middleware
) -> impl IntoResponse {
    match app_state.quiz_service.delete_quiz(quiz_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
