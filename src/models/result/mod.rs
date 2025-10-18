use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgConnection};

use crate::models::{
    question::{Question, QuestionForm},
    vec_stringify,
};

pub mod get;

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
    ) -> Result<i64, sqlx::Error> {
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
    ) -> Result<i64, sqlx::Error> {
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
    ) -> Result<i32, sqlx::Error> {
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
pub struct QuizResultSummaryQuery {
    pub user_id: i32,
    pub page: i64,
    pub size: i64,
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
pub struct AnswerResult {
    pub chosen_option_ids: Vec<(i32, bool)>, // Vec<(option_id, is_correct)>
    pub entried_text: Option<(String, bool)>, // Option<(text, is_correct)>
}

impl From<Vec<FetchedAnswer>> for AnswerResult {
    fn from(fetched_answers: Vec<FetchedAnswer>) -> Self {
        let mut chosen_option_ids = Vec::new();
        let mut entried_text = None;
        for answer in fetched_answers {
            if !answer.is_valid() {
                // return an invalid answer
                return AnswerResult {
                    chosen_option_ids: Vec::new(),
                    entried_text: None,
                };
            }
            if let Some(option_id) = answer.chosen_option_id {
                chosen_option_ids.push((option_id, answer.is_correct));
            }

            if let Some(text) = answer.entried_text {
                entried_text = Some((text, answer.is_correct));
            }
        }
        AnswerResult {
            chosen_option_ids,
            entried_text,
        }
    }
}

impl AnswerResult {
    pub fn is_valid(&self) -> bool {
        (self.chosen_option_ids.is_empty() && self.entried_text.is_some())
            || (!self.chosen_option_ids.is_empty() && self.entried_text.is_none())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionAnswerResult {
    pub question_id: i32,
    pub question: QuestionContent,
    pub answer: AnswerResult,
}

impl QuestionAnswerResult {
    pub async fn count_by_result_id(
        result_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, sqlx::Error> {
        Ok(
            sqlx::query_scalar("SELECT COUNT(*) FROM user_answers WHERE result_id = $1")
                .bind(result_id)
                .fetch_one(connection)
                .await?,
        )
    }

    fn create_from(
        fetched_question: Question,
        fetched_answers: Vec<FetchedAnswer>,
    ) -> Result<QuestionAnswerResult, &'static str> {
        let question_id = fetched_question.id;
        let question = QuestionContent::from(fetched_question.clone());
        let answer = AnswerResult::from(fetched_answers);

        if !answer.is_valid() {
            return Err("invalid answer");
        }

        for (option_id, _) in &answer.chosen_option_ids {
            if fetched_question
                .options
                .iter()
                .find(|x| &x.id == option_id)
                .is_none()
            {
                return Err("invalid option");
            }
        }

        Ok(QuestionAnswerResult {
            question_id,
            question,
            answer,
        })
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionAnswerResultQuery {
    pub result_id: i32,
    pub page: i64,
    pub size: i64,
}

#[derive(Default, Debug, FromRow)]
struct FetchedAnswer {
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
