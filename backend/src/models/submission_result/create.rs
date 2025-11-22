use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    submission_result::{DatabaseSubmissionResult, SubmissionResultCreateParams},
};

impl DatabaseSubmissionResult {
    pub async fn create_from(
        params: &SubmissionResultCreateParams,
        connection: &mut PgConnection,
    ) -> Result<DatabaseSubmissionResult, ModelError> {
        Ok(sqlx::query_as!(
            DatabaseSubmissionResult,
            r#"INSERT INTO submission_results (sub_user_id, sub_quiz_id, sub_score, sub_is_passed)
            VALUES ($1, $2, $3, $4)
            RETURNING
                sub_id AS id, sub_user_id AS "user_id!", sub_quiz_id AS "quiz_id!",
                sub_score AS "score: _", sub_is_passed AS is_passed, sub_submitted_at AS submitted_at"#,
            params.user_id,
            params.quiz_id,
            params.score,
            params.is_passed,
        )
        .fetch_one(&mut *connection)
        .await?)
    }
}
