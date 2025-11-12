use axum::{extract::{State, Json, Path, Query}, response::IntoResponse, http::StatusCode};
use crate::{
    application::services::user_service::UserService,
    interface::dto::user_dto::{UserDto, UpdateUserProfileDto, UpdateUserRoleDto, ListUsersQuery},
    interface::dto::shared_dto::{PaginatedResponse, PaginationInfo},
};

// Placeholder for application state (assuming it contains UserService)
#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    // Other services
}

/// Retrieves the profile of the currently authenticated user.
pub async fn get_me(
    State(app_state): State<AppState>,
    // In a real app, current_user_id would come from auth middleware
    current_user_id: i32, 
) -> impl IntoResponse {
    match app_state.user_service.get_user_profile(current_user_id).await {
        Ok(user) => (StatusCode::OK, Json(UserDto::from(user))).into_response(),
        Err(_) => StatusCode::NOT_FOUND.into_response(), // More specific error handling needed
    }
}

/// Updates the profile of the currently authenticated user.
pub async fn update_me(
    State(app_state): State<AppState>,
    current_user_id: i32,
    Json(dto): Json<UpdateUserProfileDto>,
) -> impl IntoResponse {
    match app_state.user_service.update_user_profile(current_user_id, dto).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(), // More specific error handling needed
    }
}

/// Admin: Retrieves a paginated list of all users.
pub async fn list_users(
    State(app_state): State<AppState>,
    Query(query): Query<ListUsersQuery>,
    // Admin role check would be done by middleware
) -> impl IntoResponse {
    match app_state.user_service.list_users(query).await {
        Ok((users, total_items, total_pages)) => {
            let user_dtos: Vec<UserDto> = users.into_iter().map(UserDto::from).collect();
            let pagination_info = PaginationInfo {
                current_page: query.page.unwrap_or(1),
                total_pages,
                total_items,
                limit: query.limit.unwrap_or(20),
            };
            (StatusCode::OK, Json(PaginatedResponse {
                data: user_dtos,
                pagination: pagination_info,
            })).into_response()
        },
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(), // More specific error handling needed
    }
}

/// Admin: Updates the role of a specific user.
pub async fn update_user_role(
    State(app_state): State<AppState>,
    Path(user_id): Path<i32>,
    Json(dto): Json<UpdateUserRoleDto>,
    // Admin role check would be done by middleware
) -> impl IntoResponse {
    match app_state.user_service.update_user_role(user_id, dto).await {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(), // More specific error handling needed
    }
}
