use std::collections::HashMap;

use serde::Deserialize;
use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    pagination::{Page, Paginate},
    question::{paginate::QuestionQuery, QuestionForm, QuestionWithKey},
    quiz::{QuizDifficulty, QuizMinimal},
    result::{
        FetchedQuizSummary, QuestionResult, QuizResultSummary, UserAnswer, UserChoice, UserEntry,
    },
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuizResultSummaryQuery {
    pub user_id: i32,
    pub page: i64,
    pub size: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuestionResultQuery {
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

        let fetched_quiz_summaries = sqlx::query_as!(
            FetchedQuizSummary,
            r#"SELECT
                r.id, r.user_id AS "user_id!", r.quiz_id AS "quiz_id!", q.title AS quiz_title, c.name AS quiz_category,
                q.difficulty AS "quiz_difficulty: QuizDifficulty", r.score, r.total_questions, r.correct_answers
            FROM results AS r JOIN quizzes AS q ON r.quiz_id = q.id JOIN categories AS c ON q.category = c.id
            WHERE r.user_id = $1 LIMIT $2 OFFSET $3"#,
            query.user_id,
            query.size,
            offset
        )
        .fetch_all(connection)
        .await?;

        let mut items = Vec::new();
        for fetched_quiz_summary in fetched_quiz_summaries {
            items.push(QuizResultSummary {
                id: fetched_quiz_summary.id,
                user_id: fetched_quiz_summary.user_id,
                quiz: QuizMinimal {
                    id: fetched_quiz_summary.quiz_id,
                    title: fetched_quiz_summary.quiz_title,
                    category: fetched_quiz_summary.quiz_category,
                    difficulty: fetched_quiz_summary.quiz_difficulty,
                },
                score: fetched_quiz_summary.score,
                total_questions: fetched_quiz_summary.total_questions,
                correct_answers: fetched_quiz_summary.correct_answers,
            });
        }

        Ok(Page::build_from(items, total_items, query.size))
    }
}

impl UserAnswer {
    async fn get_by_result_id(
        result_id: i32,
        connection: &mut PgConnection,
    ) -> Result<HashMap<i32, UserAnswer>, ModelError> {
        // HashMap<question_id, UserAnswer>
        let rows = sqlx::query!(
            r#"SELECT q.id AS question_id, a.answer_data, q.question_type AS "question_form: QuestionForm"
            FROM user_answers AS a JOIN questions AS q ON a.question_id = q.id
            WHERE result_id = $1"#,
            result_id
        )
        .fetch_all(connection)
        .await?;

        let mut answer_tuples: Vec<(i32, UserAnswer)> = Vec::new();
        for row in rows {
            match row.question_form {
                QuestionForm::MultipleChoice => {
                    let choices: Vec<UserChoice> = serde_json::from_value(row.answer_data)?;
                    answer_tuples
                        .push((row.question_id, UserAnswer::MultipleChoiceAnswer(choices)));
                }
                QuestionForm::SingleChoice => {
                    let choice: UserChoice = serde_json::from_value(row.answer_data)?;
                    answer_tuples.push((row.question_id, UserAnswer::SingleChoiceAnswer(choice)));
                }
                QuestionForm::TextEntry => {
                    let entry: UserEntry = serde_json::from_value(row.answer_data)?;
                    answer_tuples.push((row.question_id, UserAnswer::TextEntryAnswer(entry)));
                }
            }
        }

        let mut answers_map: HashMap<i32, UserAnswer> = HashMap::new();

        for (question_id, answer) in answer_tuples {
            answers_map.entry(question_id).insert_entry(answer);
        }

        Ok(answers_map)
    }
}

impl Paginate<QuestionResultQuery> for QuestionResult {
    async fn page(
        query: &QuestionResultQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let question_query = QuestionQuery {
            quiz_id: QuizResultSummary::get_quiz_id_from(query.result_id, connection).await?,
            page: query.page,
            size: query.size,
        };

        let questions_vec = QuestionWithKey::page(&question_query, connection)
            .await?
            .items;

        let mut answers_map = UserAnswer::get_by_result_id(query.result_id, connection).await?;

        let items: Vec<QuestionResult> = questions_vec
            .into_iter()
            .map(|question_with_key| {
                let user_answer = answers_map
                    .remove(&question_with_key.id)
                    .expect("invalid question id"); // take care this part later
                QuestionResult {
                    question_with_key,
                    user_answer,
                }
            })
            .collect();

        let total_items = QuestionResult::count_by_result_id(query.result_id, connection).await?;

        Ok(Page::build_from(items, total_items, query.size))
    }
}
