use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{config, models::auth::validate_access_token};

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

    let user_id = match validate_access_token(access_token, &config::secret_key()) {
        Ok(user_id) => user_id,
        Err(_) => return StatusCode::UNAUTHORIZED.into_response(),
    };

    req.extensions_mut().insert(user_id);

    next.run(req).await
}
