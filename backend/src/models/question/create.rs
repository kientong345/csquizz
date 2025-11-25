use std::str::FromStr;

use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    question::{DatabaseQuestion, QuestionCreateParams, QuestionType},
};

impl DatabaseQuestion {
    pub async fn create_from(
        params: &QuestionCreateParams,
        connection: &mut PgConnection,
    ) -> Result<DatabaseQuestion, ModelError> {
        let question_type = QuestionType::from_str(&params.r#type)?;

        Ok(sqlx::query_as!(
            DatabaseQuestion,
            r#"INSERT INTO questions (qs_quiz_id, qs_type, qs_content, qs_image_url, qs_key)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                qs_id AS id, qs_type AS "type: _", qs_content AS content, qs_image_url AS image_url,
                qs_key AS "key: _", qs_quiz_id AS "quiz_id!", qs_created_at AS created_at"#,
            params.quiz_id,
            question_type as QuestionType,
            params.content,
            params.image_url,
            params.key,
        )
        .fetch_one(connection)
        .await?)
    }
}
