use std::collections::HashMap;

use sqlx::{prelude::FromRow, PgConnection};

use crate::models::{
    pagination::{Page, Paginate},
    question::{Question, QuestionQuery},
    result::{QuizResult, QuizResultQuery, UserAnswer, UserAnswerQuery},
    vec_stringify,
};

impl Paginate<QuizResultQuery> for QuizResult {
    async fn page(
        query: &QuizResultQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, sqlx::Error> {
        let total_items = QuizResult::count_by_user_id(query.user_id, connection).await?;
        let offset = (query.page.saturating_sub(1)) * query.size;

        let items = sqlx::query_as!(
            QuizResult,
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

impl QuizResult {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuizResult, sqlx::Error> {
        Ok(sqlx::query_as!(
            QuizResult,
            r#"SELECT r.id, q.title AS quiz_title, r.score, r.total_questions, r.correct_answers
            FROM results AS r JOIN quizzes AS q ON r.quiz_id = q.id WHERE r.id = $1"#,
            id
        )
        .fetch_one(connection)
        .await?)
    }
}

#[derive(Default, Debug, FromRow)]
struct FetchedAnswer {
    chosen_options_index: Vec<i32>,
    entried_text: Option<String>,
    is_correct: Option<bool>,
}

impl FetchedAnswer {
    async fn get_by_question_ids(
        result_id: i32,
        question_ids: &[i32],
        connection: &mut PgConnection,
    ) -> Result<HashMap<i32, FetchedAnswer>, sqlx::Error> {
        todo!()
    }
}

impl Paginate<UserAnswerQuery> for UserAnswer {
    async fn page(
        query: &UserAnswerQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, sqlx::Error> {
        let quiz_id = QuizResult::get_quiz_id_from(query.result_id, connection).await?;

        let question_query = QuestionQuery {
            quiz_id,
            page: query.page,
            size: query.size,
        };

        let questions_vec = Question::page(&question_query, connection).await?.items;

        let question_ids: Vec<i32> = questions_vec.iter().map(|q| q.id).collect();

        let mut answers_map =
            FetchedAnswer::get_by_question_ids(query.result_id, &question_ids, connection).await?;

        let items: Vec<UserAnswer> = questions_vec
            .into_iter()
            .map(|q| {
                let answer = answers_map.remove(&q.id).unwrap_or_default();
                UserAnswer {
                    question_form: q.form,
                    question_text: q.text,
                    question_image_url: q.image_url,
                    options_text: vec_stringify(q.options),
                    explanation: q.explanation,
                    chosen_options_index: answer.chosen_options_index,
                    entried_text: answer.entried_text,
                    is_correct: answer.is_correct,
                }
            })
            .collect();

        let total_items = UserAnswer::count_by_result_id(query.result_id, connection).await?;

        Ok(Page::create_from(items, total_items, query.size))
    }
}
