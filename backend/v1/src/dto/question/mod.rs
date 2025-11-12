use crate::{
    models::question::{KeyType, NoKeyType, OptionKey, TextKey},
    utils::{deserialize_snake_case, serializeCamelCase},
};
use serde::{Deserialize, Serialize};

pub mod input;
pub mod output;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct OptionKeyDto {
    pub content: String,
    #[serde(
        serialize_with = "serializeCamelCase",
        deserialize_with = "deserialize_snake_case"
    )]
    pub is_correct: bool,
    pub explanation: Option<String>,
}

impl Into<OptionKeyDto> for OptionKey {
    fn into(self) -> OptionKeyDto {
        OptionKeyDto {
            content: self.content,
            is_correct: self.is_correct,
            explanation: self.explanation,
        }
    }
}

impl From<OptionKeyDto> for OptionKey {
    fn from(value: OptionKeyDto) -> Self {
        Self {
            content: value.content,
            is_correct: value.is_correct,
            explanation: value.explanation,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TextKeyDto {
    #[serde(
        serialize_with = "serializeCamelCase",
        deserialize_with = "deserialize_snake_case"
    )]
    pub correct_entry: String,
    pub explanation: Option<String>,
}

impl Into<TextKeyDto> for TextKey {
    fn into(self) -> TextKeyDto {
        TextKeyDto {
            correct_entry: self.correct_entry,
            explanation: self.explanation,
        }
    }
}

impl From<TextKeyDto> for TextKey {
    fn from(value: TextKeyDto) -> Self {
        Self {
            correct_entry: value.correct_entry,
            explanation: value.explanation,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum NoKeyTypeDto {
    SingleChoice(Vec<String>),
    MultipleChoice(Vec<String>),
    TextEntry,
}

impl Into<NoKeyTypeDto> for NoKeyType {
    fn into(self) -> NoKeyTypeDto {
        match self {
            NoKeyType::SingleChoiceKey(nokeys) => {
                let mut options = Vec::new();
                for nokey in nokeys {
                    options.push(nokey.0);
                }
                NoKeyTypeDto::SingleChoice(options)
            }
            NoKeyType::MultipleChoiceKey(nokeys) => {
                let mut options = Vec::new();
                for nokey in nokeys {
                    options.push(nokey.0);
                }
                NoKeyTypeDto::MultipleChoice(options)
            }
            NoKeyType::TextEntryKey => NoKeyTypeDto::TextEntry,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum KeyTypeDto {
    SingleChoice(Vec<OptionKeyDto>),
    MultipleChoice(Vec<OptionKeyDto>),
    TextEntry(TextKeyDto),
}

impl Into<KeyTypeDto> for KeyType {
    fn into(self) -> KeyTypeDto {
        match self {
            KeyType::SingleChoiceKey(keys) => {
                let mut options = Vec::new();
                for key in keys {
                    options.push(key.into());
                }
                KeyTypeDto::SingleChoice(options)
            }
            KeyType::MultipleChoiceKey(keys) => {
                let mut options = Vec::new();
                for key in keys {
                    options.push(key.into());
                }
                KeyTypeDto::MultipleChoice(options)
            }
            KeyType::TextEntryKey(key) => KeyTypeDto::TextEntry(key.into()),
        }
    }
}
