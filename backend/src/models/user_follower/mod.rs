use sqlx::prelude::FromRow;

pub mod create;
pub mod delete;
pub mod get;

#[derive(Debug, Clone, FromRow)]
pub struct DatabaseUserFollower {
    pub follower_id: i32,
    pub followed_id: i32,
}
