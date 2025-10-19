use sqlx::PgConnection;

use crate::models::{result::QuizResultSummary, submission::SubmissionResult};

impl QuizResultSummary {
    pub async fn create(
        submission_result: SubmissionResult,
        connection: &mut PgConnection,
    ) -> Result<QuizResultSummary, sqlx::Error> {
        todo!()
    }
}
