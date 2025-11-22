use sqlx::PgConnection;

use crate::models::{answer::Answer, error::ModelError};

impl Answer {
    pub async fn get_by_id(id: i32, connection: &mut PgConnection) -> Result<Answer, ModelError> {
        Ok(sqlx::query_as!(
            Answer,
            r#"SELECT
                ans_id AS id, ans_result_id AS "result_id!", ans_question_id AS "question_id!",
                ans_is_correct AS is_correct, ans_data AS data
            FROM answers
            WHERE ans_id = $1"#,
            id
        )
        .fetch_one(connection)
        .await?)
    }

    pub async fn count_by_result_id(
        result_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "count!"
            FROM answers
            WHERE ans_result_id = $1"#,
            result_id
        )
        .fetch_one(connection)
        .await?)
    }

    pub async fn correct_count_by_result_id(
        result_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "count!"
            FROM answers
            WHERE ans_result_id = $1 AND ans_is_correct IS TRUE"#,
            result_id
        )
        .fetch_one(connection)
        .await?)
    }
}
