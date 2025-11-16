use axum::{
    extract::{Json, Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
};

use crate::features::shared::app_state::AppState;

// /// Retrieves a paginated list of questions for a specific quiz.
// pub async fn list_questions(
//     State(app_state): State<AppState>,
//     Query(query): Query<ListQuestionsQuery>,
// ) -> impl IntoResponse {
//     match app_state.question_service.list_questions(query).await {
//         Ok((questions, total_items, total_pages)) => {
//             let question_dtos: Vec<QuestionDto> =
//                 questions.into_iter().map(QuestionDto::from).collect();
//             let pagination_info = PaginationInfo {
//                 current_page: query.page.unwrap_or(1),
//                 total_pages,
//                 total_items,
//                 limit: query.limit.unwrap_or(10),
//             };
//             (
//                 StatusCode::OK,
//                 Json(PaginatedResponse {
//                     data: question_dtos,
//                     pagination: pagination_info,
//                 }),
//             )
//                 .into_response()
//         }
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
// }

// /// Admin: Creates a new question for a quiz.
// pub async fn create_question(
//     State(app_state): State<AppState>,
//     Json(dto): Json<CreateQuestionDto>,
//     // Admin role check by middleware
// ) -> impl IntoResponse {
//     match app_state.question_service.create_question(dto).await {
//         Ok(question) => (StatusCode::CREATED, Json(QuestionDto::from(question))).into_response(),
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
// }

// /// Retrieves details of a specific question.
// pub async fn get_question(
//     State(app_state): State<AppState>,
//     Path(question_id): Path<i32>,
// ) -> impl IntoResponse {
//     match app_state
//         .question_service
//         .get_question_by_id(question_id)
//         .await
//     {
//         Ok(Some(question)) => (StatusCode::OK, Json(QuestionDto::from(question))).into_response(),
//         Ok(None) => StatusCode::NOT_FOUND.into_response(),
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
// }

// /// Admin: Updates an existing question.
// pub async fn update_question(
//     State(app_state): State<AppState>,
//     Path(question_id): Path<i32>,
//     Json(dto): Json<UpdateQuestionDto>,
//     // Admin role check by middleware
// ) -> impl IntoResponse {
//     match app_state
//         .question_service
//         .update_question(question_id, dto)
//         .await
//     {
//         Ok(question) => (StatusCode::OK, Json(QuestionDto::from(question))).into_response(),
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
// }

// /// Admin: Deletes a question.
// pub async fn delete_question(
//     State(app_state): State<AppState>,
//     Path(question_id): Path<i32>,
//     // Admin role check by middleware
// ) -> impl IntoResponse {
//     match app_state
//         .question_service
//         .delete_question(question_id)
//         .await
//     {
//         Ok(_) => StatusCode::NO_CONTENT.into_response(),
//         Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
//     }
// }
