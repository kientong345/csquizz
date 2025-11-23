use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sqlx::prelude::FromRow;

use crate::models::error::ModelError;

pub mod create;
pub mod delete;
pub mod get;
pub mod paginate;
pub mod update;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "question_type", rename_all = "snake_case")]
pub enum QuestionType {
    SingleChoice,
    MultipleChoice,
    TextEntry,
}

impl ToString for QuestionType {
    fn to_string(&self) -> String {
        match self {
            QuestionType::MultipleChoice => "multiple_choice".to_string(),
            QuestionType::SingleChoice => "single_choice".to_string(),
            QuestionType::TextEntry => "text_entry".to_string(),
        }
    }
}

impl FromStr for QuestionType {
    type Err = ModelError;

    fn from_str(input: &str) -> Result<QuestionType, Self::Err> {
        match input {
            "multiple_choice" => Ok(QuestionType::MultipleChoice),
            "single_choice" => Ok(QuestionType::SingleChoice),
            "text_entry" => Ok(QuestionType::TextEntry),
            _ => Err(ModelError::BadPost(input.to_string())),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionKey {
    pub id: i32,
    pub content: String,
    pub image_url: Option<String>,
    pub is_correct: bool,
    pub explanation: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionKeys {
    pub keys: Vec<OptionKey>,
}

impl TryFrom<Value> for OptionKeys {
    type Error = ModelError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let seriallized_key: OptionKeys = serde_json::from_value(value)?;
        Ok(seriallized_key)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionPubKey {
    pub id: i32,
    pub content: String,
    pub image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptionPubKeys {
    pub keys: Vec<OptionPubKey>,
}

impl From<OptionKeys> for OptionPubKeys {
    fn from(value: OptionKeys) -> Self {
        let keys = value
            .keys
            .into_iter()
            .map(|e| OptionPubKey {
                id: e.id,
                content: e.content,
                image_url: e.image_url,
            })
            .collect();

        Self { keys }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextKey {
    pub correct_entry: String,
    pub explanation: Option<String>,
}

impl TryFrom<Value> for TextKey {
    type Error = ModelError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let seriallized_key: TextKey = serde_json::from_value(value)?;
        Ok(seriallized_key)
    }
}

#[derive(Debug, Clone)]
pub enum KeyType {
    SingleChoiceKey(OptionKeys),
    MultipleChoiceKey(OptionKeys),
    TextEntryKey(TextKey),
}

#[derive(Debug, Clone, FromRow)]
pub struct DatabaseQuestion {
    pub id: i32,
    pub r#type: QuestionType,
    pub content: String,
    pub image_url: Option<String>,
    #[sqlx(json)]
    pub key: Value,
    pub quiz_id: i32,
    pub created_at: Option<DateTime<Utc>>,
}

impl From<DatabaseQuestionAlter> for DatabaseQuestion {
    fn from(value: DatabaseQuestionAlter) -> Self {
        let (r#type, key) = match value.key {
            KeyType::MultipleChoiceKey(keys) => (QuestionType::MultipleChoice, json!(keys)),
            KeyType::SingleChoiceKey(keys) => (QuestionType::SingleChoice, json!(keys)),
            KeyType::TextEntryKey(key) => (QuestionType::TextEntry, json!(key)),
        };
        Self {
            id: value.id,
            r#type,
            content: value.content,
            image_url: value.image_url,
            key,
            quiz_id: value.quiz_id,
            created_at: value.created_at,
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct DatabaseQuestionAlter {
    pub id: i32,
    pub content: String,
    pub image_url: Option<String>,
    #[sqlx(json)]
    pub key: KeyType,
    pub quiz_id: i32,
    pub created_at: Option<DateTime<Utc>>,
}

impl TryFrom<DatabaseQuestion> for DatabaseQuestionAlter {
    type Error = ModelError;
    fn try_from(value: DatabaseQuestion) -> Result<Self, Self::Error> {
        let key = match value.r#type {
            QuestionType::MultipleChoice => {
                KeyType::MultipleChoiceKey(OptionKeys::try_from(value.key)?)
            }
            QuestionType::SingleChoice => {
                KeyType::SingleChoiceKey(OptionKeys::try_from(value.key)?)
            }
            QuestionType::TextEntry => KeyType::TextEntryKey(TextKey::try_from(value.key)?),
        };
        Ok(Self {
            id: value.id,
            content: value.content,
            image_url: value.image_url,
            key,
            quiz_id: value.quiz_id,
            created_at: value.created_at,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionPublicData {
    pub id: i32,
    pub r#type: String,
    pub content: String,
    pub image_url: Option<String>,
    pub public_data: Value,
    pub quiz_id: i32,
    pub created_at: Option<String>,
}

impl TryFrom<QuestionPrivateData> for QuestionPublicData {
    type Error = ModelError;
    fn try_from(value: QuestionPrivateData) -> Result<Self, Self::Error> {
        let public_data = match QuestionType::from_str(&value.r#type)? {
            QuestionType::MultipleChoice => {
                let public_data =
                    OptionPubKeys::from(OptionKeys::try_from(value.private_data.clone())?);
                json!(public_data)
            }
            QuestionType::SingleChoice => {
                let public_data =
                    OptionPubKeys::from(OptionKeys::try_from(value.private_data.clone())?);
                json!(public_data)
            }
            QuestionType::TextEntry => {
                json!("")
            }
        };
        Ok(Self {
            id: value.id,
            r#type: value.r#type,
            content: value.content,
            image_url: value.image_url,
            public_data,
            quiz_id: value.quiz_id,
            created_at: value.created_at,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionPrivateData {
    pub id: i32,
    pub r#type: String,
    pub content: String,
    pub image_url: Option<String>,
    pub private_data: Value,
    pub quiz_id: i32,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuestionCreateParams {
    pub quiz_id: i32,
    pub r#type: String,
    pub content: String,
    pub image_url: Option<String>,
    pub key: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuestionUpdateParams {
    pub id: i32,
    pub r#type: Option<String>,
    pub content: Option<String>,
    pub image_url: Option<String>,
    pub key: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuestionPaginateParams {
    pub quiz_id: i32,
    pub page: i32,
    pub page_size: i32,
}

impl KeyType {
    pub fn validate(self) -> Result<Self, ModelError> {
        match &self {
            KeyType::MultipleChoiceKey(_) => Ok(self),
            KeyType::SingleChoiceKey(k) => {
                let mut correct_option_count: u8 = 0;
                for key in &k.keys {
                    if key.is_correct {
                        correct_option_count += 1;
                        if correct_option_count > 1 {
                            return Err(ModelError::BadPost(
                                "only one correct answer allowed".to_string(),
                            ));
                        }
                    }
                }
                if correct_option_count == 0 {
                    return Err(ModelError::BadPost(
                        "one correct answer must be provided".to_string(),
                    ));
                }
                Ok(self)
            }
            KeyType::TextEntryKey(_) => Ok(self),
        }
    }
}
