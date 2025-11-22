use crate::models::question::DatabaseQuestion;

impl DatabaseQuestion {
    pub async fn delete_by(
        question_id: i32,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), crate::models::error::ModelError> {
        sqlx::query!(r#"DELETE FROM questions WHERE qs_id = $1"#, question_id)
            .execute(connection)
            .await?;

        Ok(())
    }
}
