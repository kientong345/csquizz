use std::str::FromStr;

use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    pagination::{Page, Paginate},
    quiz::QuizDifficulty,
    submission_result::{
        SubmissionResultMinimal, SubmissionResultPaginateParams, SubmissionResultSortField,
    },
};

impl Paginate<SubmissionResultPaginateParams> for SubmissionResultMinimal {
    async fn page(
        params: &SubmissionResultPaginateParams,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let offset = (params.page.saturating_sub(1)) * params.page_size;

        let quiz_title_pattern = format!("%{}%", params.quiz_title_pattern.as_str());
        let pass_only_judge = if params.passed_only { 1 } else { 0 };
        let quiz_difficulty = if let Some(diff) = &params.quiz_difficulty {
            Some(QuizDifficulty::from_str(&diff)?)
        } else {
            None
        };
        let order = match SubmissionResultSortField::from_str(&params.sort_by)? {
            SubmissionResultSortField::HighestScore => String::from("rs.sub_score"),
            SubmissionResultSortField::LatestSubmission => String::from("rs.sub_submitted_at"),
        };

        let items = sqlx::query_as!(
            SubmissionResultMinimal,
            r#"SELECT
                rs.sub_id AS id, rs.sub_score AS "score: _", rs.sub_is_passed AS is_passed,
                rs.sub_submitted_at AS "submitted_at: _", q.qz_title AS quiz_title
            FROM submission_results AS rs
            INNER JOIN quizzes AS q
            ON rs.sub_quiz_id = q.qz_id
            WHERE
                rs.sub_user_id = $1 AND
                q.qz_title ILIKE $2 AND
                (rs.sub_is_passed = TRUE OR 1 = $3) AND
                q.qz_difficulty = $4
            ORDER BY $5
            OFFSET $6 LIMIT $7"#,
            params.user_id,
            &quiz_title_pattern,
            pass_only_judge,
            quiz_difficulty.clone() as Option<QuizDifficulty>,
            order,
            offset as i64,
            params.page_size as i64,
        )
        .fetch_all(&mut *connection)
        .await?;

        let total_items = sqlx::query_scalar!(
            r#"SELECT COUNT(*)
            FROM submission_results AS rs
            INNER JOIN quizzes AS q
            ON rs.sub_quiz_id = q.qz_id
            WHERE
                rs.sub_user_id = $1 AND
                q.qz_title ILIKE $2 AND
                (rs.sub_is_passed = TRUE OR 1 = $3) AND
                q.qz_difficulty = $4"#,
            params.user_id,
            &quiz_title_pattern,
            pass_only_judge,
            quiz_difficulty as Option<QuizDifficulty>,
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0);

        Ok(Page::build_from(
            items,
            total_items,
            params.page as i64,
            params.page_size,
        ))
    }
}
