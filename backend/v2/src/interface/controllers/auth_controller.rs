use axum::{extract::{State, Json}, response::IntoResponse, http::StatusCode};
use crate::{
    application::services::auth_service::AuthService,
    interface::dto::user_dto::{RegisterUserDto, LoginDto, GoogleLoginDto, LoginResponseDto},
};

// Placeholder for application state
#[derive(Clone)]
pub struct AppState {
    pub auth_service: AuthService,
    // Other services will go here
}

/// Handles user registration.
pub async fn register(
    State(app_state): State<AppState>,
    Json(dto): Json<RegisterUserDto>,
) -> impl IntoResponse {
    match app_state.auth_service.register(dto).await {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR, // More specific error handling needed
    }
}

/// Handles user login.
pub async fn login(
    State(app_state): State<AppState>,
    Json(dto): Json<LoginDto>,
) -> impl IntoResponse {
    match app_state.auth_service.login(dto).await {
        Ok(token) => (StatusCode::OK, Json(LoginResponseDto {
            token_type: "Bearer".to_string(),
            access_token: token,
        })).into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(), // More specific error handling needed
    }
}

/// Handles Google login/registration.
pub async fn google_login(
    State(app_state): State<AppState>,
    Json(dto): Json<GoogleLoginDto>,
) -> impl IntoResponse {
    match app_state.auth_service.google_login(dto).await {
        Ok(token) => (StatusCode::OK, Json(LoginResponseDto {
            token_type: "Bearer".to_string(),
            access_token: token,
        })).into_response(),
        Err(_) => StatusCode::UNAUTHORIZED.into_response(), // More specific error handling needed
    }
}
