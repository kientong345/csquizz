use sqlx::PgConnection;

use crate::models::{
    comment::{CreateCommentParams, DatabaseComment},
    error::ModelError,
};

impl DatabaseComment {
    pub async fn create_from(
        params: &CreateCommentParams,
        connection: &mut PgConnection,
    ) -> Result<DatabaseComment, ModelError> {
        Ok(sqlx::query_as!(
            DatabaseComment,
            r#"INSERT INTO comments (cmt_user_id, cmt_quiz_id, cmt_content)
            VALUES ($1, $2, $3)
            RETURNING
                cmt_id AS id, cmt_content AS content, cmt_user_id AS "user_id!",
                cmt_quiz_id AS "quiz_id!", cmt_created_at AS "created_at!""#,
            params.user_id,
            params.quiz_id,
            params.content,
        )
        .fetch_one(connection)
        .await?)
    }
}
