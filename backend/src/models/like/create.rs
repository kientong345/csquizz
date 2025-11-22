use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    like::{DatabaseCommentLike, DatabaseQuizLike},
};

impl DatabaseQuizLike {
    pub async fn create_from(
        user_id: i32,
        quiz_id: i32,
        connection: &mut PgConnection,
    ) -> Result<DatabaseQuizLike, ModelError> {
        Ok(sqlx::query_as!(
            DatabaseQuizLike,
            r#"INSERT INTO quiz_likes (qzlk_user_id, qzlk_quiz_id)
            VALUES ($1, $2)
            RETURNING qzlk_user_id AS "user_id!", qzlk_quiz_id AS "quiz_id!""#,
            user_id,
            quiz_id
        )
        .fetch_one(connection)
        .await?)
    }
}

impl DatabaseCommentLike {
    pub async fn create_from(
        user_id: i32,
        comment_id: i32,
        connection: &mut PgConnection,
    ) -> Result<DatabaseCommentLike, ModelError> {
        Ok(sqlx::query_as!(
            DatabaseCommentLike,
            r#"INSERT INTO comment_likes (cmlk_user_id, cmlk_comment_id)
            VALUES ($1, $2)
            RETURNING cmlk_user_id AS "user_id!", cmlk_comment_id AS "comment_id!""#,
            user_id,
            comment_id
        )
        .fetch_one(connection)
        .await?)
    }
}
