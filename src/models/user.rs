use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::{FromRow, Type}, PgConnection};

use crate::models::paginate::Paginate;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserQuery {
    pub page: i32,
    pub size: i32,
}

impl Paginate<UserQuery> for User {
    async fn page(
        query: &UserQuery,
        connection: &mut PgConnection,
    ) -> Result<super::paginate::Page<Self>, sqlx::Error> {
        todo!()
    }
}

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "user_role", rename_all = "kebab-case")]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserDetail {
    pub user: User,
    pub google_id: Option<String>,
    pub email: String,
    pub password_hash: Option<String>,
    pub role: UserRole,
    pub create_at: DateTime<Utc>,
}

#[derive(Debug, FromRow)]
struct FetchedUser {
    id: i32,
    google_id: Option<String>,
    username: String,
    email: String,
    password_hash: Option<String>,
    avatar_url: Option<String>,
    role: UserRole,
    // created_at: DateTime<Utc>,
}

impl UserDetail {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<UserDetail, sqlx::Error> {
        // sqlx::query_as!(
        //     FetchedUser,
        //     r#"SELECT id, google_id, username, email, password_hash, avatar_url, role
        //     FROM users WHERE id = $1"#,
        //     id
        // ).fetch_one(connection).await?;

        todo!()
    }
}
