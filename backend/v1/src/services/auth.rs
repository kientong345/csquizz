use serde::{Deserialize, Serialize, de::DeserializeOwned};
use sqlx::PgConnection;

use crate::{
    config::auth::AuthConfig,
    models::user::{UserFullDetail, post::PostUser},
    services::error::ServiceError,
    utils::{bcrypt_hash, decode_jwt, generate_jwt, validate_email_name},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RegisterSchema {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct LoginSchema {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct OAuthSchema {
    pub google_id: String,
    pub display_name: String,
    pub email: String,
}

impl RegisterSchema {
    pub fn validate(&self) -> Result<&Self, ServiceError> {
        if let Err(e) = validate_email_name(&self.email) {
            return Err(ServiceError::InvalidAuthRequest(format!(
                "Invalid email name: {}",
                &e
            )));
        }

        Ok(self)
    }
}

impl LoginSchema {
    pub fn validate(&self) -> Result<&Self, ServiceError> {
        if let Err(e) = validate_email_name(&self.email) {
            return Err(ServiceError::InvalidAuthRequest(format!(
                "Invalid email name: {}",
                &e
            )));
        }

        Ok(self)
    }
}

pub struct JwtMachine {
    config: AuthConfig,
}

impl JwtMachine {
    pub fn init(config: &AuthConfig) -> JwtMachine {
        JwtMachine {
            config: config.clone(),
        }
    }

    pub fn decode<C: Clone + DeserializeOwned>(&self, jwt: &str) -> Result<C, ServiceError> {
        let secret = self.config.jwt_secret.as_bytes().to_vec();
        Ok(decode_jwt::<C>(jwt, &secret)?)
    }

    pub fn generate_token_pair(&self, user: &UserFullDetail) -> (String, String) {
        let now = chrono::Utc::now();
        let secret = self.config.jwt_secret.as_bytes().to_vec();

        let access_claims = AccessClaims {
            sub: user.pub_info.id.to_string(),
            role: user.pub_info.role.to_string(),
            exp: (now + chrono::Duration::minutes(self.config.access_token_expire_min)).timestamp(),
        };

        let access_token = generate_jwt(&access_claims, &secret);

        let refresh_claims = RefreshClaims {
            sub: user.pub_info.id.to_string(),
            exp: (now + chrono::Duration::minutes(self.config.refresh_token_expire_min))
                .timestamp(),
        };

        let refresh_token = generate_jwt(&refresh_claims, &secret);

        (access_token, refresh_token)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessClaims {
    pub sub: String,
    pub role: String,
    pub exp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: i64,
}

pub struct AuthenticatedUser(UserFullDetail);

impl Into<UserFullDetail> for AuthenticatedUser {
    fn into(self) -> UserFullDetail {
        self.0
    }
}

impl AuthenticatedUser {
    pub async fn register(
        registration: RegisterSchema,
        connection: &mut PgConnection,
    ) -> Result<AuthenticatedUser, ServiceError> {
        if UserFullDetail::is_email_exist(&registration.email, connection).await? {
            return Err(ServiceError::EmailTaken {
                email: registration.email,
            });
        }
        let password_hash = bcrypt_hash(&registration.password)?;
        let post_user = PostUser {
            google_id: None,
            display_name: registration.display_name,
            email: registration.email,
            password_hash: Some(password_hash),
            avatar_url: None,
            role: None, // default is "user"
        };
        let user = UserFullDetail::create_from(post_user, connection).await?;
        Ok(AuthenticatedUser(user))
    }

    pub async fn login(
        login_form: LoginSchema,
        connection: &mut PgConnection,
    ) -> Result<AuthenticatedUser, ServiceError> {
        let user =
            UserFullDetail::validate_login(&login_form.email, &login_form.password, connection)
                .await?;
        Ok(AuthenticatedUser(user))
    }

    pub async fn login_by_google(
        oauth: OAuthSchema,
        connection: &mut PgConnection,
    ) -> Result<AuthenticatedUser, ServiceError> {
        let user = if !UserFullDetail::is_email_exist(&oauth.email, connection).await? {
            let post_user = PostUser {
                google_id: Some(oauth.google_id),
                display_name: oauth.display_name,
                email: oauth.email,
                password_hash: None,
                avatar_url: None,
                role: None, // default is "user"
            };
            UserFullDetail::create_from(post_user, connection).await?
        } else {
            UserFullDetail::get_by_email(&oauth.email, connection).await?
        };

        Ok(AuthenticatedUser(user))
    }
}
