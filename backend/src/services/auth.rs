use serde::de::DeserializeOwned;
use sqlx::PgConnection;

use crate::{
    config::auth::AuthConfig,
    models::{
        auth::{AccessClaims, LoginSchema, OAuthSchema, RefreshClaims, RegisterSchema},
        user::{DatabaseUser, UserCreateParams},
    },
    services::error::ServiceError,
    utils::{bcrypt_hash, decode_jwt, generate_jwt},
};

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

    pub fn generate_token_pair(&self, user: &DatabaseUser) -> (String, String) {
        let now = chrono::Utc::now();
        let secret = self.config.jwt_secret.as_bytes().to_vec();

        let access_claims = AccessClaims {
            sub: user.id.to_string(),
            role: user.role.to_string(),
            exp: (now + chrono::Duration::minutes(self.config.access_token_expire_min)).timestamp(),
        };

        let access_token = generate_jwt(&access_claims, &secret);

        let refresh_claims = RefreshClaims {
            sub: user.id.to_string(),
            exp: (now + chrono::Duration::minutes(self.config.refresh_token_expire_min))
                .timestamp(),
        };

        let refresh_token = generate_jwt(&refresh_claims, &secret);

        (access_token, refresh_token)
    }
}

pub struct AuthenticatedUser(DatabaseUser);

impl Into<DatabaseUser> for AuthenticatedUser {
    fn into(self) -> DatabaseUser {
        self.0
    }
}

impl AuthenticatedUser {
    pub async fn register(
        registration: RegisterSchema,
        connection: &mut PgConnection,
    ) -> Result<AuthenticatedUser, ServiceError> {
        if DatabaseUser::is_email_exist(&registration.email, connection).await? {
            return Err(ServiceError::EmailTaken {
                email: registration.email,
            });
        }
        let password_hash = bcrypt_hash(&registration.password)?;
        let post_user = UserCreateParams {
            google_id: None,
            display_name: registration.display_name,
            email: registration.email,
            password_hash: Some(password_hash),
            avatar_url: None,
            role: None, // default is "user"
        };
        let user = DatabaseUser::create_from(post_user, connection).await?;
        Ok(AuthenticatedUser(user))
    }

    pub async fn login(
        login_form: LoginSchema,
        connection: &mut PgConnection,
    ) -> Result<AuthenticatedUser, ServiceError> {
        let user =
            DatabaseUser::validate_login(&login_form.email, &login_form.password, connection)
                .await?;
        Ok(AuthenticatedUser(user))
    }

    pub async fn login_by_google(
        oauth: OAuthSchema,
        connection: &mut PgConnection,
    ) -> Result<AuthenticatedUser, ServiceError> {
        let user = if !DatabaseUser::is_email_exist(&oauth.email, connection).await? {
            let post_user = UserCreateParams {
                google_id: Some(oauth.google_id),
                display_name: oauth.display_name,
                email: oauth.email,
                password_hash: None,
                avatar_url: None,
                role: None, // default is "user"
            };
            DatabaseUser::create_from(post_user, connection).await?
        } else {
            DatabaseUser::get_by_email(&oauth.email, connection).await?
        };

        Ok(AuthenticatedUser(user))
    }
}
