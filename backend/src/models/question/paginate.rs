use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    pagination::{Page, Paginate},
    question::{QuestionPaginateParams, QuestionPrivateData},
};

impl Paginate<QuestionPaginateParams> for QuestionPrivateData {
    async fn page(
        params: &QuestionPaginateParams,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let offset = (params.page.saturating_sub(1)) * params.page_size;

        let items = sqlx::query_as!(
            QuestionPrivateData,
            r#"SELECT
                qs_id AS id, qs_type AS "type: _", qs_content AS content, qs_image_url AS image_url,
                qs_key AS "private_data: serde_json::Value", qs_quiz_id AS "quiz_id!", qs_created_at AS "created_at: _"
            FROM questions
            WHERE qs_quiz_id = $1
            OFFSET $2 LIMIT $3"#,
            params.quiz_id,
            offset as i64,
            params.page_size as i64,
        )
        .fetch_all(&mut *connection)
        .await?;

        let total_items = sqlx::query_scalar!(
            r#"SELECT COUNT (*) FROM questions WHERE qs_quiz_id = $1"#,
            params.quiz_id
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
