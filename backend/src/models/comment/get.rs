use crate::models::comment::DatabaseComment;

impl DatabaseComment {
    pub async fn count_by_quiz_id(
        quiz_id: i32,
        connection: &mut sqlx::PgConnection,
    ) -> Result<i64, crate::models::error::ModelError> {
        Ok(sqlx::query_scalar!(
            "SELECT COUNT(*) FROM comments WHERE cmt_quiz_id = $1",
            quiz_id
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0))
    }
}
