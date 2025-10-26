use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::PgConnection;

use crate::{
    models::{error::ModelError, user::UserFullDetail},
    utils::validate_email_name,
};

#[derive(Debug, Deserialize)]
pub struct Registration {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

impl Registration {
    pub fn validate(&self) -> Result<&Self, ModelError> {
        if let Err(e) = validate_email_name(&self.email) {
            return Err(ModelError::InvalidAuthRequest(format!(
                "Invalid email name: {}",
                &e
            )));
        }

        Ok(self)
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

impl LoginForm {
    pub fn validate(&self) -> Result<&Self, ModelError> {
        if let Err(e) = validate_email_name(&self.email) {
            return Err(ModelError::InvalidAuthRequest(format!(
                "Invalid email name: {}",
                &e
            )));
        }

        Ok(self)
    }
}

#[derive(Debug, Deserialize)]
pub struct OAuthPayload {
    pub google_id: String,
    pub display_name: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub enum SignupMethod {
    WithPassword(Registration),
    OAuth(OAuthPayload),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,
    pub role: String,
    pub exp: i64,
}

impl AccessClaims {
    pub fn decode(jwt: &str, secret: &[u8]) -> Result<Self, ModelError> {
        Ok(decode_jwt::<AccessClaims>(jwt, secret)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: i64,
}

impl RefreshClaims {
    pub fn decode(jwt: &str, secret: &[u8]) -> Result<Self, ModelError> {
        Ok(decode_jwt::<RefreshClaims>(jwt, secret)?)
    }
}

const ACCESS_TOKEN_EXPIRE: i64 = 15;
const REFRESH_TOKEN_EXPIRE: i64 = 7 * 24 * 60;

fn generate_jwt<C: Serialize>(claims: &C, secret: &[u8]) -> String {
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(&secret),
    )
    .unwrap_or(String::from(""))
}

pub fn generate_token_pair(user: &UserFullDetail, secret: &[u8]) -> (String, String) {
    let now = chrono::Utc::now();

    let access_claims = AccessClaims {
        sub: user.pub_info.id.to_string(),
        role: user.pub_info.role.to_string(),
        exp: (now + chrono::Duration::minutes(ACCESS_TOKEN_EXPIRE)).timestamp(),
    };

    let access_token = generate_jwt(&access_claims, secret);

    let refresh_claims = RefreshClaims {
        sub: user.pub_info.id.to_string(),
        exp: (now + chrono::Duration::minutes(REFRESH_TOKEN_EXPIRE)).timestamp(),
    };

    let refresh_token = generate_jwt(&refresh_claims, secret);

    (access_token, refresh_token)
}

fn decode_jwt<C: Clone + DeserializeOwned>(jwt: &str, secret: &[u8]) -> Result<C, ModelError> {
    Ok(jsonwebtoken::decode::<C>(
        jwt,
        &jsonwebtoken::DecodingKey::from_secret(&secret),
        &jsonwebtoken::Validation::default(),
    )?
    .claims)
}

pub struct AuthenticatedUser(UserFullDetail);

impl Into<UserFullDetail> for AuthenticatedUser {
    fn into(self) -> UserFullDetail {
        self.0
    }
}

impl AuthenticatedUser {
    pub async fn register(
        registration: Registration,
        connection: &mut PgConnection,
    ) -> Result<AuthenticatedUser, ModelError> {
        if UserFullDetail::is_email_exist(&registration.email, connection).await? {
            return Err(ModelError::EmailTaken {
                email: registration.email,
            });
        }
        let signup_method = SignupMethod::WithPassword(registration);
        let user = UserFullDetail::create_from(signup_method, connection).await?;
        Ok(AuthenticatedUser(user))
    }

    pub async fn login(
        login_form: LoginForm,
        connection: &mut PgConnection,
    ) -> Result<AuthenticatedUser, ModelError> {
        let user = UserFullDetail::validate_login(&login_form, connection).await?;
        Ok(AuthenticatedUser(user))
    }

    pub async fn login_by_google(
        oauth: OAuthPayload,
        connection: &mut PgConnection,
    ) -> Result<AuthenticatedUser, ModelError> {
        let user = if !UserFullDetail::is_email_exist(&oauth.email, connection).await? {
            let signup_method = SignupMethod::OAuth(oauth);
            UserFullDetail::create_from(signup_method, connection).await?
        } else {
            UserFullDetail::get_by_email(&oauth.email, connection).await?
        };

        Ok(AuthenticatedUser(user))
    }
}
