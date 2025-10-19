use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    PgConnection,
};

use crate::models::auth::LoginForm;

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
    pub username: String,
    pub avatar_url: Option<String>,
    pub role: UserRole,
    pub quiz_created_count: i64,
    pub quiz_completed_count: i64,
}

#[derive(Debug, Type, Deserialize, Serialize)]
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
pub struct User {
    pub pub_info: UserMinimal,
    pub email: String,
    pub password_hash: Option<String>,
    pub google_id: Option<String>,
}

impl User {
    fn create_from(
        fetched_user: FetchedUser,
        quiz_created_count: i64,
        quiz_completed_count: i64,
    ) -> User {
        User {
            pub_info: UserMinimal {
                id: fetched_user.id,
                username: fetched_user.username,
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

    pub async fn count(connection: &mut PgConnection) -> Result<i64, sqlx::Error> {
        Ok(sqlx::query_scalar("SELECT COUNT(*) FROM users")
            .fetch_one(connection)
            .await?)
    }

    pub async fn is_email_exist(
        email: &str,
        connection: &mut PgConnection,
    ) -> Result<bool, sqlx::Error> {
        Ok(sqlx::query_scalar!(
            r#"SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)"#,
            email
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(false))
    }

    pub async fn validate_login(
        login_form: &LoginForm,
        connection: &mut PgConnection,
    ) -> Result<User, sqlx::Error> {
        let user = User::get_by_email(&login_form.email, connection).await?;
        let hash = user.password_hash.as_deref().unwrap_or("");
        if bcrypt::verify(&login_form.password, hash).unwrap_or(false) {
            Ok(user)
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
}

struct FetchedUser {
    id: i32,
    username: String,
    avatar_url: Option<String>,
    role: UserRole,
    email: String,
    password_hash: Option<String>,
    google_id: Option<String>,
}
