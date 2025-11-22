use sqlx::prelude::FromRow;

pub mod create;
pub mod delete;
pub mod get;

#[derive(Debug, Clone, FromRow)]
pub struct DatabaseQuizLike {
    pub user_id: i32,
    pub quiz_id: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct DatabaseCommentLike {
    pub user_id: i32,
    pub comment_id: i32,
}
