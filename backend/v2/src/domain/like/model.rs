use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct QuizLike {
    pub user_id: i32,
    pub quiz_id: i32,
}

#[derive(Debug, Clone, FromRow)]
pub struct CommentLike {
    pub user_id: i32,
    pub comment_id: i32,
}
