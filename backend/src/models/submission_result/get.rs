use sqlx::PgConnection;

use crate::models::{
    answer::Answer,
    error::ModelError,
    question::DatabaseQuestion,
    quiz::DatabaseQuiz,
    submission_result::{DatabaseSubmissionResult, SubmissionResultDetail},
    user::DatabaseUser,
};

impl DatabaseSubmissionResult {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<DatabaseSubmissionResult, ModelError> {
        Ok(sqlx::query_as!(
            DatabaseSubmissionResult,
            r#"SELECT
                sub_id AS id, sub_user_id AS "user_id!", sub_quiz_id AS "quiz_id!", sub_score AS "score: _",
                sub_is_passed AS is_passed, sub_submitted_at AS "submitted_at: _"
            FROM submission_results
            WHERE sub_id = $1"#,
            id
        ).fetch_one(connection).await?)
    }

    pub async fn count_by_user_id(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "count!" FROM submission_results WHERE sub_user_id = $1"#,
            user_id
        )
        .fetch_one(connection)
        .await?)
    }

    pub async fn passed_count_by_user_id(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "count!" FROM submission_results WHERE sub_user_id = $1 AND sub_is_passed = TRUE"#,
            user_id
        )
        .fetch_one(connection)
        .await?)
    }
}

impl SubmissionResultDetail {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<SubmissionResultDetail, ModelError> {
        let db_result = DatabaseSubmissionResult::get_by_id(id, connection).await?;

        let quiz_title = DatabaseQuiz::get_by_id(db_result.quiz_id, connection)
            .await?
            .title;
        let owner_name = DatabaseUser::get_by_id(db_result.user_id, connection)
            .await?
            .display_name;
        let question_count =
            DatabaseQuestion::count_by_quiz_id(db_result.quiz_id, connection).await?;
        let answer_count = Answer::count_by_result_id(id, connection).await?;
        let correct_count = Answer::correct_count_by_result_id(id, connection).await?;

        Ok(Self {
            id,
            user_id: db_result.user_id,
            quiz_id: db_result.quiz_id,
            score: db_result.score,
            is_passed: db_result.is_passed,
            submitted_at: db_result.submitted_at.map(|dt| dt.to_rfc3339()),
            quiz_title,
            owner_name,
            question_count,
            answer_count,
            correct_count,
        })
    }
}
