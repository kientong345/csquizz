use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;
use std::sync::Arc;

use crate::models::auth::AccessClaims; // Assuming AccessClaims is public

// You should have a shared AppState that holds the secret
pub struct AppState {
    pub jwt_secret: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: String,
}

// The extractor that will carry the claims
#[derive(Debug, Clone)]
pub struct AuthClaims(pub AccessClaims);

#[async_trait]
impl<S> FromRequestParts<S> for AuthClaims
where
    S: Send + Sync,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // For this to work, you must add the secret to your app state.
        // Example in main.rs:
        // let shared_state = Arc::new(AppState { jwt_secret: "your_secret".to_string() });
        // let app = create_router().with_state(shared_state);

        // This part is tricky without seeing your main.rs, so I'm making an assumption
        // that the secret is available via an extension or state.
        // A better approach is to pass the secret directly to the middleware layer.
        // Let's assume for now we can't access state here directly and need to improve it.

        // For now, let's read from an environment variable as a placeholder.
        // THIS IS NOT IDEAL for production. You should load it into state at startup.
        let secret = std::env::var("JWT_SECRET").map_err(|_| AuthError::MissingSecret)?;

        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        let token = if let Some(header) = auth_header {
            if let Some(token) = header.strip_prefix("Bearer ") {
                token
            } else {
                return Err(AuthError::InvalidToken);
            }
        } else {
            return Err(AuthError::MissingToken);
        };

        let claims = decode::<AccessClaims>(
            token,
            &DecodingKey::from_secret(secret.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| AuthError::InvalidToken)?
        .claims;

        Ok(AuthClaims(claims))
    }
}

pub enum AuthError {
    MissingToken,
    InvalidToken,
    MissingSecret,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::MissingToken => (StatusCode::UNAUTHORIZED, "Missing authentication token"),
            AuthError::InvalidToken => (StatusCode::UNAUTHORIZED, "Invalid authentication token"),
            AuthError::MissingSecret => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "JWT secret not configured",
            ),
        };

        let body = Json(ErrorResponse {
            message: error_message.to_string(),
            status: status.to_string(),
        });

        (status, body).into_response()
    }
}
