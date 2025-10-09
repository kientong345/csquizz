use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection};

use crate::models::paginate::{Page, Paginate};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnswerOption {
    pub id: i32,
    pub text: String,
}

#[derive(FromRow)]
struct FetchedAnswerOption {
    id: i32,
    text: String,
    question_id: i32,
}

impl AnswerOption {
    pub async fn get_by_question_ids(
        question_ids: &[i32],
        connection: &mut PgConnection,
    ) -> Result<HashMap<i32, Vec<AnswerOption>>, sqlx::Error> {
        if question_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let fetched_options = sqlx::query_as::<_, FetchedAnswerOption>(
            "SELECT id, text, question_id FROM answer_options WHERE question_id = ANY($1)",
        )
        .bind(question_ids)
        .fetch_all(connection)
        .await?;

        let mut options_map: HashMap<i32, Vec<AnswerOption>> = HashMap::new();
        for fetched in fetched_options {
            options_map
                .entry(fetched.question_id)
                .or_default()
                .push(AnswerOption {
                    id: fetched.id,
                    text: fetched.text,
                });
        }
        Ok(options_map)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Question {
    pub id: i32,
    pub form: String,
    pub text: String,
    pub image_url: Option<String>,
    pub options: Vec<AnswerOption>,
}

impl Question {
    pub async fn count_by_quiz_id(
        quiz_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, sqlx::Error> {
        Ok(
            sqlx::query_scalar("SELECT COUNT(*) FROM questions WHERE quiz_id = $1")
                .bind(quiz_id)
                .fetch_one(&mut *connection)
                .await?,
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionQuery {
    pub quiz_id: i32,
    pub page: i64,
    pub size: i64,
}

struct FetchedQuestion {
    id: i32,
    form: String,
    text: String,
    image_url: Option<String>,
}

impl FetchedQuestion {
    async fn get_by_query(
        query: &QuestionQuery,
        connection: &mut PgConnection,
    ) -> Result<Vec<FetchedQuestion>, sqlx::Error> {
        let offset = (query.page.saturating_sub(1)) * query.size;
        Ok(sqlx::query_as!(
            FetchedQuestion,
            r#"SELECT id, question_type AS form, question_text AS text, image_url FROM questions LIMIT $1 OFFSET $2"#,
            query.size,
            offset
        )
        .fetch_all(connection).await?)
    }

    fn into_full_options(self, options: Vec<AnswerOption>) -> Question {
        Question {
            id: self.id,
            form: self.form,
            text: self.text,
            image_url: self.image_url,
            options,
        }
    }
}

impl Paginate<QuestionQuery> for Question {
    async fn page(
        query: &QuestionQuery,
        connection: &mut PgConnection,
    ) -> Result<super::paginate::Page<Self>, sqlx::Error> {
        let fetched_questions = FetchedQuestion::get_by_query(query, connection).await?;

        let question_ids_with_options: Vec<i32> = fetched_questions
            .iter()
            .filter(|q| q.form != "text-entry")
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

        Ok(Page::create_from(items, total_items, query.size))
    }
}
