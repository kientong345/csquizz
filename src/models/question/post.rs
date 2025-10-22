use serde::Deserialize;
use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    question::{AnswerOption, FetchedQuestion, Question, QuestionForm},
};

#[derive(Debug, Deserialize)]
pub struct PostOption {
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Deserialize)]
pub enum PostQuestionForm {
    MultipleChoice(Vec<PostOption>),
    SingleChoice(Vec<PostOption>),
    TextEntry(String), // correct_entry
}

#[derive(Debug, Deserialize)]
pub struct PostQuestion {
    pub quiz_id: i32,
    pub text: String,
    pub image_url: Option<String>,
    pub explanation: Option<String>,
    pub form: PostQuestionForm,
}

impl Question {
    pub async fn create_from(
        data: PostQuestion,
        connection: &mut PgConnection,
    ) -> Result<Question, ModelError> {
        let (form, options_to_insert, correct_entry) = match data.form {
            PostQuestionForm::MultipleChoice(options) => {
                (String::from("multiple-choice"), Some(options), None)
            }
            PostQuestionForm::SingleChoice(options) => {
                (String::from("single-choice"), Some(options), None)
            }
            PostQuestionForm::TextEntry(correct_text) => {
                (String::from("text-entry"), None, Some(correct_text))
            }
        };

        let fetched_question = sqlx::query_as!(
            FetchedQuestion,
            r#"INSERT INTO questions (quiz_id, question_type, question_text, image_url, correct_entry, explanation)
            VALUES ($1, $2::text::question_form, $3, $4, $5, $6)
            RETURNING id, question_type AS "form: QuestionForm", question_text AS text, image_url, explanation"#,
            data.quiz_id,
            form,
            data.text,
            data.image_url,
            correct_entry,
            data.explanation,
        ).fetch_one(&mut *connection).await?;

        let mut options = Vec::new();
        if let Some(inserted_options) = options_to_insert {
            for option in inserted_options {
                let inserted_option = sqlx::query_as!(
                    AnswerOption,
                    r#"INSERT INTO options (question_id, option_text, is_correct)
                    VALUES ($1, $2, $3) RETURNING id, option_text AS text"#,
                    fetched_question.id,
                    option.text,
                    option.is_correct,
                )
                .fetch_one(&mut *connection)
                .await?;

                options.push(inserted_option);
            }
        }

        Ok(fetched_question.into_full_options(options))
    }
}
