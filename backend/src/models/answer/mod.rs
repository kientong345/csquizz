use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;

use crate::models::error::ModelError;

pub mod create;
pub mod get;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserChoice {
    pub option_id: i32,
}

impl TryFrom<Value> for UserChoice {
    type Error = ModelError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let seriallized_choice: UserChoice = serde_json::from_value(value)?;
        Ok(seriallized_choice)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserChoices {
    pub choices: Vec<UserChoice>,
}

impl TryFrom<Value> for UserChoices {
    type Error = ModelError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let seriallized_choice: UserChoices = serde_json::from_value(value)?;
        Ok(seriallized_choice)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntry {
    pub entry: String,
}

impl TryFrom<Value> for UserEntry {
    type Error = ModelError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let seriallized_entry: UserEntry = serde_json::from_value(value)?;
        Ok(seriallized_entry)
    }
}

#[derive(Debug, Clone, FromRow, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Answer {
    pub id: i32,
    pub result_id: i32,
    pub question_id: i32,
    pub is_correct: bool,
    #[sqlx(json)]
    pub data: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnswerCreateParams {
    pub result_id: i32,
    pub question_id: i32,
    pub is_correct: bool,
    pub data: Value,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct UnevaluatedAnswer {
    pub question_id: i32,
    pub data: Value,
}

impl UnevaluatedAnswer {
    pub fn bind(self, is_correct: bool) -> EvaluatedAnswer {
        EvaluatedAnswer {
            question_id: self.question_id,
            data: self.data,
            is_correct,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct EvaluatedAnswer {
    pub question_id: i32,
    pub data: Value,
    pub is_correct: bool,
}

impl EvaluatedAnswer {
    pub fn bind(self, result_id: i32) -> AnswerCreateParams {
        AnswerCreateParams {
            result_id,
            question_id: self.question_id,
            is_correct: self.is_correct,
            data: self.data,
        }
    }
}
