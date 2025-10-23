use std::collections::HashMap;

use sqlx::{prelude::FromRow, PgConnection};

use crate::models::{
    error::ModelError,
    question::{AnswerOption, FetchedQuestion, Question, QuestionForm},
};

#[derive(Debug, FromRow)]
struct FetchedAnswerOption {
    id: i32,
    option_text: String,
    question_id: i32,
}

impl AnswerOption {
    pub async fn get_by_question_ids(
        question_ids: &[i32],
        connection: &mut PgConnection,
    ) -> Result<HashMap<i32, Vec<AnswerOption>>, ModelError> {
        if question_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let fetched_options = sqlx::query_as::<_, FetchedAnswerOption>(
            "SELECT id, option_text, question_id FROM options WHERE question_id = ANY($1)",
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
                    text: fetched.option_text,
                });
        }
        Ok(options_map)
    }
}

impl Question {
    pub async fn get_by_id(
        question_id: i32,
        connection: &mut PgConnection,
    ) -> Result<Question, ModelError> {
        let fetched_question = sqlx::query_as!(
            FetchedQuestion,
            r#"SELECT id, question_type AS "form: QuestionForm", question_text AS text, image_url, explanation FROM questions
            WHERE id = $1"#,
            question_id,
        ).fetch_one(&mut *connection).await?;

        let options = sqlx::query_as!(
            AnswerOption,
            "SELECT id, option_text AS text FROM options WHERE question_id = $1",
            question_id,
        )
        .fetch_all(connection)
        .await?;

        Ok(Question {
            id: question_id,
            form: fetched_question.form,
            text: fetched_question.text,
            image_url: fetched_question.image_url,
            explanation: fetched_question.explanation,
            options,
        })
    }
}
