use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection};

use crate::{
    models::{
        error::ModelError,
        question::{Question, QuestionForm},
    },
    utils::vec_stringify,
};

pub mod get;
pub mod paginate;
pub mod post;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct QuizResultSummary {
    pub id: i32,
    pub quiz_title: String,
    pub score: f64,
    pub total_questions: i32,
    pub correct_answers: i32,
    // pub submitted_at: DateTime<Utc>,
}

impl QuizResultSummary {
    pub async fn count_by_user_id(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(
            sqlx::query_scalar("SELECT COUNT(*) FROM results WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(connection)
                .await?,
        )
    }

    pub async fn count_distinct_by_user_id(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(
            sqlx::query_scalar("SELECT COUNT(DISTINCT user_id) FROM results WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(connection)
                .await?,
        )
    }

    pub async fn get_quiz_id_from(
        result_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i32, ModelError> {
        Ok(
            sqlx::query!(r#"SELECT quiz_id FROM results WHERE id = $1"#, result_id)
                .fetch_one(connection)
                .await?
                .quiz_id
                .unwrap_or(-1),
        )
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionContent {
    pub question_form: QuestionForm,
    pub question_text: String,
    pub question_image_url: Option<String>,
    pub options_text: Vec<String>,
    pub explanation: Option<String>,
}

impl From<Question> for QuestionContent {
    fn from(value: Question) -> Self {
        QuestionContent {
            question_form: value.form,
            question_text: value.text,
            question_image_url: value.image_url,
            options_text: vec_stringify(value.options),
            explanation: value.explanation,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum AnswerResultType {
    ChoicesResult(Vec<(i32, bool)>), // Vec<(option_id, is_correct)>
    TextResult(String, bool),        // Option<(text, is_correct)>
    InvalidResult,
}

impl From<Vec<FetchedAnswer>> for AnswerResultType {
    fn from(fetched_answers: Vec<FetchedAnswer>) -> Self {
        let mut chosen_option_ids = Vec::new();
        let mut entried_text = None;
        for answer in fetched_answers {
            if !answer.is_valid() {
                return AnswerResultType::InvalidResult;
            }
            if let Some(option_id) = answer.chosen_option_id {
                chosen_option_ids.push((option_id, answer.is_correct));
            }

            if let Some(text) = answer.entried_text {
                entried_text = Some((text, answer.is_correct));
            }
        }

        if (chosen_option_ids.is_empty() && entried_text.is_some())
            || (!chosen_option_ids.is_empty() && entried_text.is_none())
        {
            return AnswerResultType::InvalidResult;
        }

        if let Some((text, is_correct)) = entried_text {
            AnswerResultType::TextResult(text, is_correct)
        } else {
            AnswerResultType::ChoicesResult(chosen_option_ids)
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionAnswerResult {
    pub question_id: i32,
    pub question: QuestionContent,
    pub answer: AnswerResultType,
}

impl QuestionAnswerResult {
    pub async fn count_by_result_id(
        result_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(
            sqlx::query_scalar("SELECT COUNT(*) FROM user_answers WHERE result_id = $1")
                .bind(result_id)
                .fetch_one(connection)
                .await?,
        )
    }

    fn build_from(
        fetched_question: Question,
        fetched_answers: Vec<FetchedAnswer>,
    ) -> Result<QuestionAnswerResult, &'static str> {
        let question_id = fetched_question.id;
        let question = QuestionContent::from(fetched_question.clone());
        let answer = AnswerResultType::from(fetched_answers);

        match &answer {
            AnswerResultType::InvalidResult => return Err("invalid answer"),
            AnswerResultType::ChoicesResult(choices_result) => {
                for (option_id, _) in choices_result {
                    if fetched_question
                        .options
                        .iter()
                        .find(|x| &x.id == option_id)
                        .is_none()
                    {
                        return Err("invalid option");
                    }
                }
            }
            _ => (),
        }

        Ok(QuestionAnswerResult {
            question_id,
            question,
            answer,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct QuizResult {
    pub summary: QuizResultSummary,
    pub result: Vec<QuestionAnswerResult>,
}

#[derive(Default, Debug, FromRow)]
struct FetchedAnswer {
    question_id: Option<i32>,
    chosen_option_id: Option<i32>,
    entried_text: Option<String>,
    is_correct: bool,
}

impl FetchedAnswer {
    fn is_valid(&self) -> bool {
        (self.chosen_option_id.is_some() && self.entried_text.is_none())
            || (self.chosen_option_id.is_none() && self.entried_text.is_some())
    }
}
