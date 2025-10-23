use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{config, models::auth::AccessClaims};

pub async fn auth_middleware(mut req: Request<Body>, next: Next) -> Response {
    let auth_header = match req.headers().get("Authorization") {
        Some(value) => value,
        None => return StatusCode::NON_AUTHORITATIVE_INFORMATION.into_response(),
    };

    let access_token = auth_header
        .to_str()
        .unwrap()
        .strip_prefix("Bearer ")
        .unwrap();

    let access_claims = match AccessClaims::decode(access_token, &config::secret_key()) {
        Ok(access_claims) => access_claims,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    req.extensions_mut().insert(access_claims);

    next.run(req).await
}
