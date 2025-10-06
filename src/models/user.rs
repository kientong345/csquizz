use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{pool::PoolConnection, prelude::FromRow, Postgres};

use crate::models::paginate::Paginate;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    id: i32,
    username: String,
    avatar_url: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UserQuery {}

impl Paginate<UserQuery> for User {
    async fn page(
        query: &UserQuery,
        connection: &PoolConnection<Postgres>,
    ) -> Result<super::paginate::Page<Self>, sqlx::Error> {
        todo!()
    }
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct UserDetail {
    user: User,
    google_id: Option<String>,
    email: String,
    password_hash: Option<String>,
    role: String,
    create_at: DateTime<Utc>,
}

impl UserDetail {
    pub async fn get_by_id(
        id: i32,
        connection: &PoolConnection<Postgres>,
    ) -> Result<UserDetail, sqlx::Error> {
        todo!()
    }
}
