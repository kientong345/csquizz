use reqwest::Url;
use serde::Deserialize;

use crate::{config::oauth::OAuthConfig, models::auth::OAuthSchema, services::error::ServiceError};

pub struct OAuthClient {
    client: reqwest::Client,
    config: OAuthConfig,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OAuthResponse {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct GoogleUserResult {
    pub id: String,
    pub email: String,
    pub verified_email: bool,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub locale: String,
}

impl Into<OAuthSchema> for GoogleUserResult {
    fn into(self) -> OAuthSchema {
        OAuthSchema {
            google_id: self.id,
            display_name: self.name,
            email: self.email,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AuthorizationCode {
    pub code: String,
    pub state: String,
}

impl OAuthClient {
    pub fn init(config: &OAuthConfig) -> OAuthClient {
        OAuthClient {
            client: reqwest::Client::new(),
            config: config.clone(),
        }
    }

    pub async fn request_token(
        &self,
        authorization_code: &str,
    ) -> Result<OAuthResponse, ServiceError> {
        let params = [
            ("grant_type", "authorization_code"),
            ("redirect_uri", self.config.redirect_url.as_str()),
            ("client_id", self.config.google_client_id.as_str()),
            ("code", authorization_code),
            ("client_secret", self.config.google_client_secret.as_str()),
        ];
        let response = self
            .client
            .post(self.config.token_url.as_str())
            .form(&params)
            .send()
            .await?;

        let oauth_response = response.json::<OAuthResponse>().await?;
        Ok(oauth_response)
    }

    pub async fn get_google_user(
        &self,
        access_token: &str,
        id_token: &str,
    ) -> Result<GoogleUserResult, ServiceError> {
        let mut url = Url::parse(&self.config.user_info_url).unwrap();
        url.query_pairs_mut().append_pair("alt", "json");
        url.query_pairs_mut()
            .append_pair("access_token", access_token);

        let response = self.client.get(url).bearer_auth(id_token).send().await?;

        let user_info = response.json::<GoogleUserResult>().await?;
        Ok(user_info)
    }
}
