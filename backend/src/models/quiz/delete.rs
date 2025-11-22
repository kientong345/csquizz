use crate::models::quiz::DatabaseQuiz;

impl DatabaseQuiz {
    pub async fn delete_by(
        quiz_id: i32,
        connection: &mut sqlx::PgConnection,
    ) -> Result<(), crate::models::error::ModelError> {
        sqlx::query!(r#"DELETE FROM quizzes WHERE qz_id = $1"#, quiz_id)
            .execute(connection)
            .await?;

        Ok(())
    }
}
