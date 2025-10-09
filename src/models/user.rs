use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection};

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

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserDetail {
    pub user: User,
    pub google_id: Option<String>,
    pub email: String,
    pub password_hash: Option<String>,
    pub role: String,
    pub create_at: DateTime<Utc>,
}

impl UserDetail {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<UserDetail, sqlx::Error> {
        todo!()
    }
}
