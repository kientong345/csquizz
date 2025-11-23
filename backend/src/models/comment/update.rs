use crate::models::comment::{CommentUpdateParams, DatabaseComment};

impl DatabaseComment {
    pub async fn update_by(
        id: i32,
        params: &CommentUpdateParams,
        connection: &mut sqlx::PgConnection,
    ) -> Result<DatabaseComment, crate::models::error::ModelError> {
        Ok(sqlx::query_as!(
            DatabaseComment,
            r#"UPDATE comments
            SET cmt_content = $1
            WHERE cmt_id = $2
            RETURNING
                cmt_id AS id, cmt_content AS content, cmt_user_id AS "user_id!",
                cmt_quiz_id AS "quiz_id!", cmt_created_at AS "created_at!""#,
            params.content,
            id
        )
        .fetch_one(connection)
        .await?)
    }
}
