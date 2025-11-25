use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
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
        oauth::AuthorizationCode,
    },
    services::oauth_client::OAuthClient,
};

pub async fn handle_register(
    State(state): State<AppState>,
    Json(registration): Json<RegisterSchema>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    registration.validate()?;

    state
        .auth_service
        .register(&mut *connection, registration)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::CREATED)
}

pub async fn handle_login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(login_form): Json<LoginSchema>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    login_form.validate()?;

    let (_, access_token, refresh_token) = state
        .auth_service
        .login(&mut *connection, login_form)
        .await?;

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

// pub async fn handle_google_login(
//     State(state): State<AppState>,
//     jar: CookieJar,
//     Query(auth_code) : Query<AuthorizationCode>,
// ) -> Result<(CookieJar, Json<Value>), ControllerError> {
//     // let client = &state.read().await.client;

//     // let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
//     // let (auth_url, csrf_state) = client
//     //     .authorize_url(CsrfToken::new_random)
//     //     .add_scope(Scope::new("".to_string()))
//     //     .set_pkce_challenge(pkce_challenge)
//     //     .url();

//     // Ok(Redirect::to(auth_url.as_str()))

//     todo!()
// }

pub async fn handle_google_login(
    State(state): State<AppState>,
    Query(auth_code): Query<AuthorizationCode>,
) -> Result<(CookieJar, Json<Value>), ControllerError> {
    let oauth_client = OAuthClient::init(&state.config.oauth_config);

    let token_response = oauth_client.exchange_for_token(&auth_code.code).await?;

    let google_user = oauth_client
        .get_google_user(&token_response.access_token, &token_response.id_token)
        .await?;

    let mut connection = state.primary_db.start_transaction().await?;

    let (_, access_token, refresh_token) = state
        .auth_service
        .google_login(&mut *connection, google_user.into())
        .await?;

    connection.commit().await?;

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
