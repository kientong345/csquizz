use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    pagination::{Page, Paginate},
    question::{paginate::QuestionQuery, Question},
    result::{FetchedAnswer, QuestionAnswerResult, QuizResultSummary},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizResultSummaryQuery {
    pub user_id: i32,
    pub page: i64,
    pub size: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionAnswerResultQuery {
    pub result_id: i32,
    pub page: i64,
    pub size: i64,
}

impl Paginate<QuizResultSummaryQuery> for QuizResultSummary {
    async fn page(
        query: &QuizResultSummaryQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let total_items = QuizResultSummary::count_by_user_id(query.user_id, connection).await?;
        let offset = (query.page.saturating_sub(1)) * query.size;

        let items = sqlx::query_as!(
            QuizResultSummary,
            r#"SELECT r.id, q.title AS quiz_title, r.score, r.total_questions, r.correct_answers
            FROM results AS r JOIN quizzes AS q ON r.quiz_id = q.id WHERE r.user_id = $1 LIMIT $2 OFFSET $3"#,
            query.user_id,
            query.size,
            offset
        )
        .fetch_all(connection)
        .await?;

        Ok(Page::build_from(items, total_items, query.size))
    }
}

impl FetchedAnswer {
    async fn get_by_result_id(
        result_id: i32,
        connection: &mut PgConnection,
    ) -> Result<HashMap<i32, Vec<FetchedAnswer>>, ModelError> {
        // HashMap<question_id, Vec<FetchedAnswer>
        let fetched_answers = sqlx::query_as!(
            FetchedAnswer,
            r#"SELECT question_id, selected_option AS chosen_option_id, entried_text, is_correct
            FROM user_answers WHERE result_id = $1"#,
            result_id
        )
        .fetch_all(connection)
        .await?;

        let mut answers_map: HashMap<i32, Vec<FetchedAnswer>> = HashMap::new();

        for answer in fetched_answers {
            answers_map
                .entry(answer.question_id.unwrap())
                .or_default()
                .push(answer);
        }

        Ok(answers_map)
    }
}

impl Paginate<QuestionAnswerResultQuery> for QuestionAnswerResult {
    async fn page(
        query: &QuestionAnswerResultQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let question_query = QuestionQuery {
            quiz_id: QuizResultSummary::get_quiz_id_from(query.result_id, connection).await?,
            page: query.page,
            size: query.size,
        };

        let questions_vec = Question::page(&question_query, connection).await?.items;

        let mut answers_map = FetchedAnswer::get_by_result_id(query.result_id, connection).await?;

        let items: Vec<QuestionAnswerResult> = questions_vec
            .into_iter()
            .map(|q| {
                let answers = answers_map.remove(&q.id).unwrap_or_default();
                QuestionAnswerResult::build_from(q, answers).unwrap()
            })
            .collect();

        let total_items =
            QuestionAnswerResult::count_by_result_id(query.result_id, connection).await?;

        Ok(Page::build_from(items, total_items, query.size))
    }
}
