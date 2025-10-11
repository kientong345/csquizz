use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    PgConnection,
};

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

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar_url: Option<String>,
    pub email: String,
    pub role: UserRole,
}

impl User {
    pub async fn is_name_taken(
        name: &str,
        connection: &mut PgConnection,
    ) -> Result<bool, sqlx::Error> {
        todo!()
    }
}
