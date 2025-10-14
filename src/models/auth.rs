use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::PgConnection;

use crate::models::user::User;

#[derive(Debug, Deserialize)]
pub struct Registration {
    pub username: String,
    pub password: String,
    pub email: String,
}

impl Registration {
    pub fn is_valid(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

impl LoginForm {
    pub fn is_valid(&self) -> bool {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
pub struct OAuthPayload {
    pub google_id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize)]
pub enum SignupMethod {
    WithPassword(Registration),
    OAuth(OAuthPayload),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AccessClaims {
    sub: String,
    role: String,
    exp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RefreshClaims {
    sub: String,
    exp: i64,
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

pub fn generate_token_pair(user: &User, secret: &[u8]) -> (String, String) {
    let now = chrono::Utc::now();

    let access_claims = AccessClaims {
        sub: user.id.to_string(),
        role: user.role.to_string(),
        exp: (now + chrono::Duration::minutes(ACCESS_TOKEN_EXPIRE)).timestamp(),
    };

    let access_token = generate_jwt(&access_claims, secret);

    let refresh_claims = RefreshClaims {
        sub: user.id.to_string(),
        exp: (now + chrono::Duration::minutes(REFRESH_TOKEN_EXPIRE)).timestamp(),
    };

    let refresh_token = generate_jwt(&refresh_claims, secret);

    (access_token, refresh_token)
}

fn decode_jwt<C: Clone + DeserializeOwned>(
    jwt: &str,
    secret: &[u8],
) -> Result<C, jsonwebtoken::errors::Error> {
    Ok(jsonwebtoken::decode::<C>(
        jwt,
        &jsonwebtoken::DecodingKey::from_secret(&secret),
        &jsonwebtoken::Validation::default(),
    )?
    .claims)
}

pub fn validate_access_token(
    access_token: &str,
    secret: &[u8],
) -> Result<i32, jsonwebtoken::errors::Error> {
    Ok(decode_jwt::<AccessClaims>(access_token, secret)?
        .sub
        .parse()
        .unwrap_or(-1))
}

pub struct AuthenticatedUser(User);

impl Into<User> for AuthenticatedUser {
    fn into(self) -> User {
        self.0
    }
}

impl AuthenticatedUser {
    pub async fn register(
        registration: Registration,
        connection: &mut PgConnection,
    ) -> Result<AuthenticatedUser, sqlx::Error> {
        if User::is_email_exist(&registration.email, connection).await? {
            return Err(sqlx::Error::BeginFailed);
        }
        let signup_method = SignupMethod::WithPassword(registration);
        let user = User::create(signup_method, connection).await?;
        Ok(AuthenticatedUser(user))
    }

    pub async fn login(
        login_form: LoginForm,
        connection: &mut PgConnection,
    ) -> Result<AuthenticatedUser, sqlx::Error> {
        let user = User::validate_login(&login_form, connection).await?;
        Ok(AuthenticatedUser(user))
    }

    pub async fn login_by_google(
        oauth: OAuthPayload,
        connection: &mut PgConnection,
    ) -> Result<AuthenticatedUser, sqlx::Error> {
        let user = if !User::is_email_exist(&oauth.email, connection).await? {
            let signup_method = SignupMethod::OAuth(oauth);
            User::create(signup_method, connection).await?
        } else {
            User::get_by_email(&oauth.email, connection).await?
        };

        Ok(AuthenticatedUser(user))
    }
}
