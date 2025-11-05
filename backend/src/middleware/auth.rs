use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tokio::sync::RwLock;

use crate::{app::AppState, models::auth::AccessClaims};

pub async fn auth_middleware(
    State(state): State<Arc<RwLock<AppState>>>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let state_locked = state.read().await;

    let auth_header = match req.headers().get("Authorization") {
        Some(value) => value,
        None => return StatusCode::NON_AUTHORITATIVE_INFORMATION.into_response(),
    };

    let access_token = auth_header
        .to_str()
        .unwrap()
        .strip_prefix("Bearer ")
        .unwrap();

    let secret = state_locked
        .config
        .auth_config
        .jwt_secret
        .as_bytes()
        .to_vec();
    let access_claims = match AccessClaims::decode(access_token, &secret) {
        Ok(access_claims) => access_claims,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    req.extensions_mut().insert(access_claims);

    next.run(req).await
}
