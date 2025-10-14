use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use serde_json::{json, Value};

use crate::{
    database::pool::QuizBankPool,
    models::{
        auth::{
            generate_token_pair, validate_access_token, AuthenticatedUser, LoginForm, Registration,
        },
        user::User,
    },
    services::config,
};

pub async fn handle_register(
    State(pool): State<QuizBankPool>,
    jar: CookieJar,
    Json(registration): Json<Registration>,
) -> Result<(CookieJar, Json<Value>), StatusCode> {
    let mut connection = match pool.start_transaction().await {
        Ok(connection) => connection,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    if !registration.is_valid() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user: User = match AuthenticatedUser::register(registration, &mut connection).await {
        Ok(user) => user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    }
    .into();

    let (access_token, refresh_token) = generate_token_pair(&user, &config::secret_key());

    let cookie: Cookie = Cookie::build(refresh_token)
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .into();

    match connection.commit().await {
        Ok(_) => Ok((
            jar.add(cookie),
            Json(json!({
                "access_token": access_token,
                "user": user
            })),
        )),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn handle_login(
    State(pool): State<QuizBankPool>,
    jar: CookieJar,
    Json(login_form): Json<LoginForm>,
) -> Result<(CookieJar, Json<Value>), StatusCode> {
    let mut connection = match pool.get_connection().await {
        Ok(connection) => connection,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if !login_form.is_valid() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user: User = match AuthenticatedUser::login(login_form, &mut *connection).await {
        Ok(user) => user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    }
    .into();

    let (access_token, refresh_token) = generate_token_pair(&user, &config::secret_key());

    let cookie: Cookie = Cookie::build(refresh_token)
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .into();

    Ok((
        jar.add(cookie),
        Json(json!({
            "access_token": access_token,
            "user": user
        })),
    ))
}

pub async fn get_my_info(
    State(pool): State<QuizBankPool>,
    headers: HeaderMap,
) -> Result<Json<Value>, StatusCode> {
    let mut connection = match pool.get_connection().await {
        Ok(connection) => connection,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let access_token = auth_header.trim_start_matches("Bearer ").trim();

    let user_id = match validate_access_token(access_token, &config::secret_key()) {
        Ok(user_id) => user_id,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    match User::get_by_id(user_id, &mut *connection).await {
        Ok(user) => Ok(Json(json!(user))),
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    }
}
