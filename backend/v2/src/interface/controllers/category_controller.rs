use axum::{extract::{State, Json, Path}, response::IntoResponse, http::StatusCode};
use crate::{
    application::services::category_service::CategoryService,
    interface::dto::category_dto::{CategoryDto, CreateCategoryDto, UpdateCategoryDto},
};

// Placeholder for application state
#[derive(Clone)]
pub struct AppState {
    pub category_service: CategoryService,
    // Other services
}

/// Retrieves a list of all categories.
pub async fn list_categories(
    State(app_state): State<AppState>,
) -> impl IntoResponse {
    match app_state.category_service.list_categories().await {
        Ok(categories) => {
            let category_dtos: Vec<CategoryDto> = categories.into_iter().map(CategoryDto::from).collect();
            (StatusCode::OK, Json(category_dtos)).into_response()
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Admin: Creates a new category.
pub async fn create_category(
    State(app_state): State<AppState>,
    Json(dto): Json<CreateCategoryDto>,
    // Admin role check by middleware
) -> impl IntoResponse {
    match app_state.category_service.create_category(dto).await {
        Ok(category) => (StatusCode::CREATED, Json(CategoryDto::from(category))).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Admin: Updates an existing category.
pub async fn update_category(
    State(app_state): State<AppState>,
    Path(category_id): Path<i32>,
    Json(dto): Json<UpdateCategoryDto>,
    // Admin role check by middleware
) -> impl IntoResponse {
    match app_state.category_service.update_category(category_id, dto).await {
        Ok(category) => (StatusCode::OK, Json(CategoryDto::from(category))).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// Admin: Deletes a category.
pub async fn delete_category(
    State(app_state): State<AppState>,
    Path(category_id): Path<i32>,
    // Admin role check by middleware
) -> impl IntoResponse {
    match app_state.category_service.delete_category(category_id).await {
        Ok(_) => StatusCode::NO_CONTENT.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
