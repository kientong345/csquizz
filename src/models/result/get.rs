use std::collections::HashMap;

use sqlx::{prelude::FromRow, PgConnection};

use crate::models::{
    pagination::{Page, Paginate},
    question::{Question, QuestionQuery},
    result::{
        FetchedAnswer, QuestionAnswerResult, QuestionAnswerResultQuery, QuizResultSummary,
        QuizResultSummaryQuery,
    },
};

impl Paginate<QuizResultSummaryQuery> for QuizResultSummary {
    async fn page(
        query: &QuizResultSummaryQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, sqlx::Error> {
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

        Ok(Page::create_from(items, total_items, query.size))
    }
}

impl QuizResultSummary {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuizResultSummary, sqlx::Error> {
        Ok(sqlx::query_as!(
            QuizResultSummary,
            r#"SELECT r.id, q.title AS quiz_title, r.score, r.total_questions, r.correct_answers
            FROM results AS r JOIN quizzes AS q ON r.quiz_id = q.id WHERE r.id = $1"#,
            id
        )
        .fetch_one(connection)
        .await?)
    }
}

impl FetchedAnswer {
    async fn get_by_result_id(
        result_id: i32,
        connection: &mut PgConnection,
    ) -> Result<HashMap<i32, Vec<FetchedAnswer>>, sqlx::Error> {
        // HashMap<question_id, Vec<FetchedAnswer>
        #[derive(FromRow)]
        struct AnswerWithQuestionId {
            question_id: Option<i32>,
            selected_option: Option<i32>,
            entried_text: Option<String>,
            is_correct: bool,
        }

        let rows = sqlx::query_as!(
            AnswerWithQuestionId,
            r#"SELECT question_id, selected_option, entried_text, is_correct
            FROM user_answers WHERE result_id = $1"#,
            result_id
        )
        .fetch_all(connection)
        .await?;

        let mut answers_map: HashMap<i32, Vec<FetchedAnswer>> = HashMap::new();

        for row in rows {
            let answer = FetchedAnswer {
                chosen_option_id: row.selected_option,
                entried_text: row.entried_text,
                is_correct: row.is_correct,
            };
            answers_map
                .entry(row.question_id.unwrap())
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
    ) -> Result<Page<Self>, sqlx::Error> {
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
                QuestionAnswerResult::create_from(q, answers).unwrap()
            })
            .collect();

        let total_items =
            QuestionAnswerResult::count_by_result_id(query.result_id, connection).await?;

        Ok(Page::create_from(items, total_items, query.size))
    }
}
