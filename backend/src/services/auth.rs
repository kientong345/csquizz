use serde::de::DeserializeOwned;
use sqlx::PgConnection;

use crate::{
    config::auth::AuthConfig,
    models::{
        auth::{AccessClaims, LoginSchema, RefreshClaims, RegisterSchema},
        oauth::OAuthSchema,
        user::{DatabaseUser, UserCreateParams},
    },
    services::error::ServiceError,
    utils::{bcrypt_hash, decode_jwt, generate_jwt},
};

#[derive(Clone)]
pub struct AuthService {
    config: AuthConfig,
}

impl AuthService {
    pub fn new(config: AuthConfig) -> Self {
        Self { config }
    }

    pub fn decode_jwt<C: Clone + DeserializeOwned>(&self, jwt: &str) -> Result<C, ServiceError> {
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

    pub async fn register(
        &self,
        conn: &mut PgConnection,
        registration: RegisterSchema,
    ) -> Result<DatabaseUser, ServiceError> {
        if DatabaseUser::is_email_exist(&registration.email, conn).await? {
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
        let user = DatabaseUser::create_from(post_user, conn).await?;
        Ok(user)
    }

    pub async fn login(
        &self,
        conn: &mut PgConnection,
        login_form: LoginSchema,
    ) -> Result<(DatabaseUser, String, String), ServiceError> {
        let user =
            DatabaseUser::validate_login(&login_form.email, &login_form.password, conn).await?;

        let (access_token, refresh_token) = self.generate_token_pair(&user);

        Ok((user, access_token, refresh_token))
    }

    pub async fn google_login(
        &self,
        conn: &mut PgConnection,
        oauth: OAuthSchema,
    ) -> Result<(DatabaseUser, String, String), ServiceError> {
        let user = if !DatabaseUser::is_email_exist(&oauth.email, conn).await? {
            let post_user = UserCreateParams {
                google_id: Some(oauth.google_id),
                display_name: oauth.display_name,
                email: oauth.email,
                password_hash: None,
                avatar_url: None,
                role: None, // default is "user"
            };
            DatabaseUser::create_from(post_user, conn).await?
        } else {
            DatabaseUser::get_by_email(&oauth.email, conn).await?
        };

        let (access_token, refresh_token) = self.generate_token_pair(&user);

        Ok((user, access_token, refresh_token))
    }
}
