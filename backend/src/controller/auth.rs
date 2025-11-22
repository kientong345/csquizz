use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::Redirect,
};
use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use serde_json::{Value, json};

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        auth::{LoginSchema, RegisterSchema},
        user::DatabaseUser,
    },
    services::{
        auth::{AuthenticatedUser, JwtMachine},
        oauth_client::{AuthorizationCode, OAuthClient},
    },
};

pub async fn handle_register(
    State(state): State<AppState>,
    Json(registration): Json<RegisterSchema>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.pool.start_transaction().await?;

    registration.validate()?;

    AuthenticatedUser::register(registration, &mut connection).await?;

    connection.commit().await?;

    Ok(StatusCode::CREATED)
}

pub async fn handle_login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(login_form): Json<LoginSchema>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    let mut connection = state.pool.get_connection().await?;

    login_form.validate()?;

    let user: DatabaseUser = AuthenticatedUser::login(login_form, &mut *connection)
        .await?
        .into();

    let jwt_machine = JwtMachine::init(&state.config.auth_config);
    let (access_token, refresh_token) = jwt_machine.generate_token_pair(&user);

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
    State(state): State<AppState>,
) -> Result<Redirect, ControllerError> {
    // let client = &state.read().await.client;

    // let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    // let (auth_url, csrf_state) = client
    //     .authorize_url(CsrfToken::new_random)
    //     .add_scope(Scope::new("".to_string()))
    //     .set_pkce_challenge(pkce_challenge)
    //     .url();

    // Ok(Redirect::to(auth_url.as_str()))

    todo!()
}

pub async fn handle_oauth_callback(
    State(state): State<AppState>,
    Query(auth_code): Query<AuthorizationCode>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    let oauth_client = OAuthClient::init(&state.config.oauth_config);

    let token_response = oauth_client.request_token(&auth_code.code).await?;

    let google_user = oauth_client
        .get_google_user(&token_response.access_token, &token_response.id_token)
        .await?;

    let mut connection = state.pool.start_transaction().await?;
    let user: DatabaseUser =
        AuthenticatedUser::login_by_google(google_user.into(), &mut *connection)
            .await?
            .into();

    let jwt_machine = JwtMachine::init(&state.config.auth_config);
    let (access_token, refresh_token) = jwt_machine.generate_token_pair(&user);

    let cookie: Cookie = Cookie::build(refresh_token)
        .http_only(true)
        .secure(false)
        .same_site(SameSite::Lax)
        .path("/")
        .into();

    let jar = CookieJar::new();

    Ok((
        jar.add(cookie),
        Json(json!({
            "access_token": access_token,
        })),
    ))
}

pub async fn handle_refresh(
    State(state): State<AppState>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    todo!()
}

pub async fn handle_logout(
    State(state): State<AppState>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    todo!()
}
