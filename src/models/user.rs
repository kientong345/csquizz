use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    PgConnection,
};

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
}

impl Into<UserDetail> for FetchedUser {
    fn into(self) -> UserDetail {
        let user = User {
            id: self.id,
            username: self.username,
            avatar_url: self.avatar_url,
        };
        UserDetail {
            user,
            google_id: self.google_id,
            email: self.email,
            password_hash: self.password_hash,
            role: self.role,
        }
    }
}

impl UserDetail {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<UserDetail, sqlx::Error> {
        let fetched_user = sqlx::query_as!(
            FetchedUser,
            r#"SELECT id, google_id, username, email, password_hash, avatar_url, role AS "role: UserRole"
            FROM users WHERE id = $1"#,
            id
        ).fetch_one(connection).await?;

        Ok(fetched_user.into())
    }
}
