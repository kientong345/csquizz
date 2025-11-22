use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    like::{DatabaseCommentLike, DatabaseQuizLike},
};

impl DatabaseQuizLike {
    pub async fn delete_by(
        user_id: i32,
        quiz_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"DELETE FROM quiz_likes
            WHERE qzlk_user_id = $1 AND qzlk_quiz_id = $2"#,
            user_id,
            quiz_id
        )
        .execute(connection)
        .await?;
        Ok(())
    }
}

impl DatabaseCommentLike {
    pub async fn delete_by(
        user_id: i32,
        comment_id: i32,
        connection: &mut PgConnection,
    ) -> Result<(), ModelError> {
        sqlx::query!(
            r#"DELETE FROM comment_likes
            WHERE cmlk_user_id = $1 AND cmlk_comment_id = $2"#,
            user_id,
            comment_id
        )
        .execute(connection)
        .await?;
        Ok(())
    }
}
