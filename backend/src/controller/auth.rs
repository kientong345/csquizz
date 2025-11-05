use std::sync::Arc;

use axum::{extract::State, http::StatusCode, Json};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use serde_json::{json, Value};
use tokio::sync::RwLock;

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        auth::{generate_token_pair, AuthenticatedUser, LoginForm, Registration},
        user::UserFullDetail,
    },
};

pub async fn handle_register(
    State(state): State<Arc<RwLock<AppState>>>,
    Json(registration): Json<Registration>,
) -> Result<StatusCode, ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.start_transaction().await?;

    registration.validate()?;

    AuthenticatedUser::register(registration, &mut connection).await?;

    connection.commit().await?;

    Ok(StatusCode::CREATED)
}

pub async fn handle_login(
    State(state): State<Arc<RwLock<AppState>>>,
    jar: CookieJar,
    Json(login_form): Json<LoginForm>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.get_connection().await?;

    login_form.validate()?;

    let user: UserFullDetail = AuthenticatedUser::login(login_form, &mut *connection)
        .await?
        .into();

    let secret = state_locked
        .config
        .auth_config
        .jwt_secret
        .as_bytes()
        .to_vec();
    let (access_token, refresh_token) = generate_token_pair(&user, &secret);

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
        })),
    ))
}

pub async fn handle_login_by_google(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    let client = &state.read().await.client;

    todo!()
}

pub async fn handle_oauth_callback(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    todo!()
}

pub async fn handle_refresh(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    todo!()
}

pub async fn handle_logout(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    todo!()
}
