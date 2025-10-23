use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    pagination::{Page, Paginate},
    question::{AnswerOption, FetchedQuestion, Question, QuestionForm},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionQuery {
    pub quiz_id: i32,
    pub page: i64,
    pub size: i64,
}

impl FetchedQuestion {
    async fn get_by_query(
        query: &QuestionQuery,
        connection: &mut PgConnection,
    ) -> Result<Vec<FetchedQuestion>, ModelError> {
        let offset = (query.page.saturating_sub(1)) * query.size;
        Ok(sqlx::query_as!(
            FetchedQuestion,
            r#"SELECT id, question_type AS "form: QuestionForm", question_text AS text, image_url, explanation FROM questions LIMIT $1 OFFSET $2"#,
            query.size,
            offset
        )
        .fetch_all(connection).await?)
    }
}

impl Paginate<QuestionQuery> for Question {
    async fn page(
        query: &QuestionQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let fetched_questions = FetchedQuestion::get_by_query(query, connection).await?;

        let question_ids_with_options: Vec<i32> = fetched_questions
            .iter()
            .filter(|q| q.form != QuestionForm::TextEntry)
            .map(|q| q.id)
            .collect();

        let mut options_map =
            AnswerOption::get_by_question_ids(&question_ids_with_options, connection).await?;

        let items: Vec<Question> = fetched_questions
            .into_iter()
            .map(|q| {
                let options = options_map.remove(&q.id).unwrap_or_default();
                q.into_full_options(options)
            })
            .collect();

        let total_items = Question::count_by_quiz_id(query.quiz_id, connection).await?;

        Ok(Page::build_from(items, total_items, query.size))
    }
}
