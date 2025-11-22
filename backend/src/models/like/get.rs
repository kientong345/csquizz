use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    like::{DatabaseCommentLike, DatabaseQuizLike},
};

impl DatabaseQuizLike {
    pub async fn count_by_quiz_id(
        quiz_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            "SELECT COUNT(*) FROM quiz_likes WHERE qzlk_quiz_id = $1",
            quiz_id
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0))
    }
}

impl DatabaseCommentLike {
    pub async fn count_by_comment_id(
        comment_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            "SELECT COUNT(*) FROM comment_likes WHERE cmlk_comment_id = $1",
            comment_id
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0))
    }
}
