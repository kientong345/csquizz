use axum::{
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{app::AppState, models::auth::AccessClaims, services::auth::JwtMachine};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Response {
    let auth_header = match req.headers().get("Authorization") {
        Some(value) => value,
        None => return StatusCode::NON_AUTHORITATIVE_INFORMATION.into_response(),
    };

    let access_token = auth_header
        .to_str()
        .unwrap()
        .strip_prefix("Bearer ")
        .unwrap();

    let jwt_machine = JwtMachine::init(&state.config.auth_config);

    let access_claims = match jwt_machine.decode::<AccessClaims>(access_token) {
        Ok(access_claims) => access_claims,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    req.extensions_mut().insert(access_claims);

    next.run(req).await
}
