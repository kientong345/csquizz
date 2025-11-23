use crate::models::answer::{Answer, AnswerCreateParams};

impl Answer {
    pub async fn create_from(
        params: &AnswerCreateParams,
        connection: &mut sqlx::PgConnection,
    ) -> Result<Answer, crate::models::error::ModelError> {
        Ok(sqlx::query_as!(
            Answer,
            r#"INSERT INTO answers (ans_result_id, ans_question_id, ans_is_correct, ans_data)
            VALUES ($1, $2, $3, $4)
            RETURNING
                ans_id AS id, ans_result_id AS "result_id!", ans_question_id AS "question_id!",
                ans_is_correct AS is_correct, ans_data AS data"#,
            params.result_id,
            params.question_id,
            params.is_correct,
            params.data
        )
        .fetch_one(connection)
        .await?)
    }
}
