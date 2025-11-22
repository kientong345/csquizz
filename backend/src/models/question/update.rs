use crate::models::question::{DatabaseQuestion, QuestionUpdateParams};

impl DatabaseQuestion {
    pub async fn update_by(
        params: &QuestionUpdateParams,
        connection: &mut sqlx::PgConnection,
    ) -> Result<DatabaseQuestion, crate::models::error::ModelError> {
        Ok(sqlx::query_as!(
            DatabaseQuestion,
            r#"UPDATE questions
            SET qs_content = COALESCE($1, qs_content),
                qs_image_url = COALESCE($2, qs_image_url),
                qs_key = COALESCE($3, qs_key)
            WHERE qs_id = $4
            RETURNING
                qs_id AS id, qs_type AS "type: _", qs_content AS content, qs_image_url AS image_url,
                qs_key AS "key: serde_json::Value", qs_quiz_id AS "quiz_id!", qs_created_at AS created_at"#,
            params.content,
            params.image_url,
            params.key,
            params.id,
        )
        .fetch_one(connection)
        .await?)
    }
}
