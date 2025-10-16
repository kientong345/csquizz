use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, PgConnection};

pub mod get;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AnswerOption {
    pub id: i32,
    pub text: String,
}

impl ToString for AnswerOption {
    fn to_string(&self) -> String {
        String::from(&self.text)
    }
}

#[derive(Debug, Type, Deserialize, Serialize, PartialEq, Eq)]
#[sqlx(type_name = "question_form", rename_all = "kebab-case")]
pub enum QuestionForm {
    MultipleChoice,
    SingleChoice,
    TextEntry,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Question {
    pub id: i32,
    pub form: QuestionForm,
    pub text: String,
    pub image_url: Option<String>,
    pub explanation: Option<String>,
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
                .fetch_one(connection)
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
