use crate::utils::{deserialize_snake_case, serializeCamelCase};
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, prelude::Type};

use crate::models::error::ModelError;

pub mod get;
pub mod paginate;
pub mod post;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct OptionKey {
    pub content: String,
    #[serde(
        serialize_with = "serializeCamelCase",
        deserialize_with = "deserialize_snake_case"
    )]
    pub is_correct: bool,
    pub explanation: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct OptionContent(String);

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub struct TextKey {
    #[serde(
        serialize_with = "serializeCamelCase",
        deserialize_with = "deserialize_snake_case"
    )]
    pub correct_entry: String,
    pub explanation: Option<String>,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum KeyType {
    SingleChoiceKey(Vec<OptionKey>),
    MultipleChoiceKey(Vec<OptionKey>),
    TextEntryKey(TextKey),
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
pub enum NoKeyType {
    SingleChoiceKey(Vec<OptionContent>),
    MultipleChoiceKey(Vec<OptionContent>),
    TextEntryKey,
}

#[derive(Debug, Type, Serialize, PartialEq, Eq, Clone, Copy)]
#[sqlx(type_name = "question_form", rename_all = "kebab-case")]
pub enum QuestionForm {
    MultipleChoice,
    SingleChoice,
    TextEntry,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct QuestionNoKey {
    pub id: i32,
    pub form: QuestionForm,
    pub text: String,
    pub image_url: Option<String>,
    pub answer_no_key: NoKeyType,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct QuestionWithKey {
    pub id: i32,
    pub form: QuestionForm,
    pub text: String,
    pub image_url: Option<String>,
    pub answer_key: KeyType,
}

#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
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
