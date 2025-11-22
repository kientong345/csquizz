use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;

use crate::models::{
    answer::Answer,
    error::ModelError,
    pagination::{Page, Paginate},
    question::QuestionPrivateData,
    submission_result::SubmissionResultDetail,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AnswerQuestion {
    pub answer: Answer,
    pub question: QuestionPrivateData,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnswerQuestionPaginateParams {
    pub result_id: i32,
    pub page: i32,
    pub page_size: i32,
}

impl Paginate<AnswerQuestionPaginateParams> for AnswerQuestion {
    async fn page(
        params: &AnswerQuestionPaginateParams,
        connection: &mut sqlx::PgConnection,
    ) -> Result<Page<Self>, super::error::ModelError> {
        let offset = (params.page.saturating_sub(1)) * params.page_size;

        let items = sqlx::query_as!(
            FetchedAnswerQuestion,
            r#"SELECT
                a.ans_id AS answer_id, a.ans_question_id AS "question_id!",
                ans_result_id AS "result_id!", ans_is_correct AS answer_is_correct,
                ans_data AS answer_data, q.qs_type AS "question_type: _",
                q.qs_content AS question_content, q.qs_image_url AS question_image_url,
                q.qs_key AS question_private_data, q.qs_quiz_id AS "question_quiz_id!",
                q.qs_created_at AS "question_created_at: _"
            FROM answers AS a
            INNER JOIN questions AS q
            ON a.ans_question_id = q.qs_id
            WHERE a.ans_result_id = $1
            OFFSET $2 LIMIT $3"#,
            params.result_id,
            offset as i64,
            params.page_size as i64,
        )
        .fetch_all(&mut *connection)
        .await?
        .into_iter()
        .map(|e| e.into())
        .collect();

        let total_items = sqlx::query_scalar!(
            r#"SELECT COUNT(*)
            FROM answers AS a
            INNER JOIN questions AS q
            ON a.ans_question_id = q.qs_id
            WHERE a.ans_result_id = $1"#,
            params.result_id
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0);

        Ok(Page::build_from(items, total_items, params.page_size))
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ResultComposition {
    pub summary: SubmissionResultDetail,
    pub data: Page<AnswerQuestion>,
}

impl ResultComposition {
    pub async fn get_by(
        params: &AnswerQuestionPaginateParams,
        connection: &mut sqlx::PgConnection,
    ) -> Result<ResultComposition, ModelError> {
        let summary = SubmissionResultDetail::get_by_id(params.result_id, connection).await?;
        let data = AnswerQuestion::page(params, connection).await?;
        Ok(ResultComposition { summary, data })
    }
}

#[derive(Debug, Clone, FromRow)]
struct FetchedAnswerQuestion {
    answer_id: i32,
    question_id: i32,
    result_id: i32,
    answer_is_correct: bool,
    answer_data: Value,
    question_type: String,
    question_content: String,
    question_image_url: Option<String>,
    question_private_data: Value,
    question_quiz_id: i32,
    question_created_at: Option<String>,
}

impl Into<AnswerQuestion> for FetchedAnswerQuestion {
    fn into(self) -> AnswerQuestion {
        AnswerQuestion {
            answer: Answer {
                id: self.answer_id,
                result_id: self.result_id,
                question_id: self.question_id,
                is_correct: self.answer_is_correct,
                data: self.answer_data,
            },
            question: QuestionPrivateData {
                id: self.question_id,
                r#type: self.question_type,
                content: self.question_content,
                image_url: self.question_image_url,
                private_data: self.question_private_data,
                quiz_id: self.question_quiz_id,
                created_at: self.question_created_at,
            },
        }
    }
}
