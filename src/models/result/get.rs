use sqlx::PgConnection;

use crate::models::{error::ModelError, result::QuizResultSummary};

impl QuizResultSummary {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuizResultSummary, ModelError> {
        Ok(sqlx::query_as!(
            QuizResultSummary,
            r#"SELECT r.id, q.title AS quiz_title, r.score, r.total_questions, r.correct_answers
            FROM results AS r JOIN quizzes AS q ON r.quiz_id = q.id WHERE r.id = $1"#,
            id
        )
        .fetch_one(connection)
        .await?)
    }
}
