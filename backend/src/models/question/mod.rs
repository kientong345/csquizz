use serde::{Deserialize, Serialize};
use sqlx::{prelude::Type, PgConnection};

use crate::models::error::ModelError;

pub mod get;
pub mod paginate;
pub mod post;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct OptionKey {
    pub content: String,
    pub is_correct: bool,
    pub explanation: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct OptionContent(String);

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TextKey {
    pub correct_entry: String,
    pub explanation: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum KeyType {
    SingleChoiceKey(Vec<OptionKey>),
    MultipleChoiceKey(Vec<OptionKey>),
    TextEntryKey(TextKey),
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum NoKeyType {
    SingleChoiceKey(Vec<OptionContent>),
    MultipleChoiceKey(Vec<OptionContent>),
    TextEntryKey,
}

#[derive(Debug, Type, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
#[sqlx(type_name = "question_form", rename_all = "kebab-case")]
pub enum QuestionForm {
    MultipleChoice,
    SingleChoice,
    TextEntry,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct QuestionNoKey {
    pub id: i32,
    pub form: QuestionForm,
    pub text: String,
    pub image_url: Option<String>,
    pub answer_no_key: NoKeyType,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct QuestionWithKey {
    pub id: i32,
    pub form: QuestionForm,
    pub text: String,
    pub image_url: Option<String>,
    pub answer_key: KeyType,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum Question {
    NoKey(QuestionNoKey),
    WithKey(QuestionWithKey),
}

impl Question {
    pub async fn count_by_quiz_id(
        quiz_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM questions WHERE quiz_id = $1"#,
            quiz_id
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0))
    }
}
