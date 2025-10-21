use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use serde_json::{json, Value};

use crate::{
    config,
    controller::error::ControllerError,
    database::pool::QuizBankPool,
    models::{
        auth::{generate_token_pair, AuthenticatedUser, LoginForm, Registration},
        user::UserFullDetail,
    },
};

pub async fn handle_register(
    State(pool): State<QuizBankPool>,
    jar: CookieJar,
    Json(registration): Json<Registration>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    let mut connection = pool.start_transaction().await?;

    if !registration.is_valid() {
        return Err(ControllerError::InvalidRegistration(String::from(
            "email, display_name or password is invalid",
        )));
    }

    let user: UserFullDetail = AuthenticatedUser::register(registration, &mut connection)
        .await?
        .into();

    let (access_token, refresh_token) = generate_token_pair(&user, &config::secret_key());

    let cookie: Cookie = Cookie::build(refresh_token)
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .into();

    connection.commit().await?;

    Ok((
        jar.add(cookie),
        Json(json!({
            "access_token": access_token,
            "user": user
        })),
    ))
}

pub async fn handle_login(
    State(pool): State<QuizBankPool>,
    jar: CookieJar,
    Json(login_form): Json<LoginForm>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    let mut connection = pool.get_connection().await?;

    if !login_form.is_valid() {
        return Err(ControllerError::InvalidLoginForm(String::from(
            "email, display_name or password is invalid",
        )));
    }

    let user: UserFullDetail = AuthenticatedUser::login(login_form, &mut *connection)
        .await?
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

pub async fn handle_refresh(State(pool): State<QuizBankPool>) -> StatusCode {
    todo!()
}

pub async fn handle_logout(State(pool): State<QuizBankPool>) -> StatusCode {
    todo!()
}
