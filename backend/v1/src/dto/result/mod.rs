use crate::{
    models::result::{UserAnswer, UserChoice, UserEntry},
    utils::{deserialize_snake_case, serializeCamelCase},
};
use serde::{Deserialize, Serialize};

pub mod input;
pub mod output;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserChoiceDto {
    #[serde(
        serialize_with = "serializeCamelCase",
        deserialize_with = "deserialize_snake_case"
    )]
    pub option_index: i32,
}

impl Into<UserChoiceDto> for UserChoice {
    fn into(self) -> UserChoiceDto {
        UserChoiceDto {
            option_index: self.option_index,
        }
    }
}

impl From<UserChoiceDto> for UserChoice {
    fn from(value: UserChoiceDto) -> Self {
        Self {
            option_index: value.option_index,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserEntryDto {
    #[serde(
        serialize_with = "serializeCamelCase",
        deserialize_with = "deserialize_snake_case"
    )]
    pub text_entried: String,
}

impl Into<UserEntryDto> for UserEntry {
    fn into(self) -> UserEntryDto {
        UserEntryDto {
            text_entried: self.text_entried,
        }
    }
}

impl From<UserEntryDto> for UserEntry {
    fn from(value: UserEntryDto) -> Self {
        Self {
            text_entried: value.text_entried,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum UserAnswerDto {
    SingleChoiceAnswer(UserChoiceDto),
    MultipleChoiceAnswer(Vec<UserChoiceDto>),
    TextEntryAnswer(UserEntryDto),
}

impl Into<UserAnswerDto> for UserAnswer {
    fn into(self) -> UserAnswerDto {
        match self {
            UserAnswer::SingleChoiceAnswer(choice) => {
                UserAnswerDto::SingleChoiceAnswer(choice.into())
            }
            UserAnswer::MultipleChoiceAnswer(choices) => {
                let mut choices_dto = Vec::new();
                for choice in choices {
                    choices_dto.push(choice.into());
                }
                UserAnswerDto::MultipleChoiceAnswer(choices_dto)
            }
            UserAnswer::TextEntryAnswer(entry) => UserAnswerDto::TextEntryAnswer(entry.into()),
        }
    }
}

impl From<UserAnswerDto> for UserAnswer {
    fn from(value: UserAnswerDto) -> Self {
        match value {
            UserAnswerDto::SingleChoiceAnswer(choice) => {
                UserAnswer::SingleChoiceAnswer(choice.into())
            }
            UserAnswerDto::MultipleChoiceAnswer(choices_dto) => {
                let mut choices = Vec::new();
                for choice_dto in choices_dto {
                    choices.push(choice_dto.into());
                }
                UserAnswer::MultipleChoiceAnswer(choices)
            }
            UserAnswerDto::TextEntryAnswer(entry) => UserAnswer::TextEntryAnswer(entry.into()),
        }
    }
}
