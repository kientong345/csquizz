use std::sync::Arc;

use axum::Router;
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use tokio::sync::RwLock;

use crate::config::oauth::OAuthConfig;
use crate::config::Configuration;
use crate::database::pool::QuizBankPool;

use crate::routes;

#[derive(Clone)]
pub struct AppState {
    pub pool: QuizBankPool,
    pub client: BasicClient,
    pub config: Configuration,
}

pub async fn create_app(state: Arc<RwLock<AppState>>) -> Router {
    Router::new()
        .merge(routes::user::create_protected_route(state.clone()))
        .merge(routes::quizzes::create_route(state.clone()))
        .merge(routes::user::create_route(state.clone()))
        .merge(routes::categories::create_route(state.clone()))
        .merge(routes::admin::create_route(state.clone()))
        .merge(routes::auth::create_route(state))
}

pub fn create_oauth_client(config: &OAuthConfig) -> BasicClient {
    let client_id = ClientId::new(config.google_client_id.clone());
    let client_secret = ClientSecret::new(config.google_client_secret.clone());
    let auth_url =
        AuthUrl::new(config.auth_url.clone()).expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new(config.token_url.clone()).expect("Invalid token endpoint URL");
    let redirect_url = RedirectUrl::new(config.redirect_url.clone()).expect("Invalid redirect URL");

    BasicClient::new(client_id)
        .set_client_secret(client_secret)
        // .set_auth_uri(auth_url)
        // .set_token_uri(token_url)
        .set_redirect_uri(redirect_url)
}
