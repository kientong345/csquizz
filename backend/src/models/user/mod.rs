use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    PgConnection,
};

use crate::models::error::ModelError;

pub mod get;
pub mod paginate;
pub mod patch;
pub mod post;

#[derive(Debug, Deserialize, Serialize)]
pub enum OrderType {
    MostSolved,
    MostCreated,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserMinimal {
    pub id: i32,
    pub display_name: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserPubInfo {
    pub id: i32,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub quiz_created_count: i64,
    pub quiz_completed_count: i64,
}

#[derive(Debug, Type, Deserialize, Serialize, PartialEq, Eq)]
#[sqlx(type_name = "user_role", rename_all = "kebab-case")]
pub enum UserRole {
    User,
    Admin,
}

impl ToString for UserRole {
    fn to_string(&self) -> String {
        match self {
            UserRole::User => String::from("user"),
            UserRole::Admin => String::from("admin"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserFullDetail {
    pub pub_info: UserPubInfo,
    pub email: String,
    pub password_hash: Option<String>,
    pub google_id: Option<String>,
}

impl UserFullDetail {
    fn build_from(
        fetched_user: FetchedUser,
        quiz_created_count: i64,
        quiz_completed_count: i64,
    ) -> UserFullDetail {
        UserFullDetail {
            pub_info: UserPubInfo {
                id: fetched_user.id,
                display_name: fetched_user.display_name,
                avatar_url: fetched_user.avatar_url,
                role: fetched_user.role,
                quiz_created_count,
                quiz_completed_count,
            },
            email: fetched_user.email,
            password_hash: fetched_user.password_hash,
            google_id: fetched_user.google_id,
        }
    }

    pub async fn count(connection: &mut PgConnection) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(connection)
            .await?)
    }

    pub async fn is_email_exist(
        email: &str,
        connection: &mut PgConnection,
    ) -> Result<bool, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"#,
            email
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(false))
    }

    pub async fn validate_login(
        email: &str,
        password: &str,
        connection: &mut PgConnection,
    ) -> Result<UserFullDetail, ModelError> {
        let user = UserFullDetail::get_by_email(email, connection).await?;
        let hash = user.password_hash.as_deref().unwrap_or("");
        if bcrypt::verify(password, hash).unwrap_or(false) {
            Ok(user)
        } else {
            Err(ModelError::WrongPasswordForEmail { email: user.email })
        }
    }
}

struct FetchedUser {
    id: i32,
    display_name: String,
    avatar_url: Option<String>,
    role: UserRole,
    email: String,
    password_hash: Option<String>,
    google_id: Option<String>,
}
