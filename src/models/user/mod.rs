use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    PgConnection,
};

use crate::models::auth::LoginForm;

pub mod get;
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
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserQuery {
    pub order_by: OrderType,
    pub page: i32,
    pub size: i32,
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
    pub id: i32,
    pub username: String,
    pub avatar_url: Option<String>,
    pub email: String,
    pub role: UserRole,
}

impl User {
    pub async fn is_email_exist(
        email: &str,
        connection: &mut PgConnection,
    ) -> Result<bool, sqlx::Error> {
        todo!()
    }

    pub async fn validate_login(
        login_form: &LoginForm,
        connection: &mut PgConnection,
    ) -> Result<User, sqlx::Error> {
        let password_hash =
            bcrypt::hash(&login_form.password, bcrypt::DEFAULT_COST).expect("hashing failed");

        Ok(sqlx::query_as!(
            User,
            r#"SELECT id, username, avatar_url, email, role AS "role: UserRole"
            FROM users WHERE email = $1 AND password_hash = $2"#,
            login_form.email,
            password_hash
        )
        .fetch_one(connection)
        .await?)
    }
}
