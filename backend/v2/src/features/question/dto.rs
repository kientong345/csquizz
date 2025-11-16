use core::str;

use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use utoipa::ToSchema;

use crate::domain::question::model::{CreateQuestionParams, Question, UpdateQuestionParams};

#[derive(Debug, Clone, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PublicQuestionDto {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "singleChoice")]
    pub r#type: String,

    #[schema(example = "What is the capital of France?")]
    pub content: String,

    #[schema(example = "https://example.com/image.png")]
    pub image_url: Option<String>,

    #[schema(value_type = Object)]
    #[schema(example = json!({
        "public_data": [
            {
                "content": "Paris",
                "image_url": null,
            },
            {
                "content": "London",
                "image_url": null,
            }
        ]
    }))]
    pub public_data: Value,

    #[schema(example = 1)]
    pub quiz_id: i32,

    #[schema(example = "2024-01-01T12:00:00Z")]
    pub created_at: Option<String>,
}

impl From<Question> for PublicQuestionDto {
    fn from(value: Question) -> Self {
        let (r#type, public_data) = match value.r#type {
            crate::domain::question::model::QuestionType::SingleChoice => {
                let deserialized_key: crate::domain::question::model::OptionKeys =
                    serde_json::from_value(value.key.clone()).unwrap();
                (
                    "singleChoice".to_string(),
                    json!(PublicOptionsDto {
                        keys: deserialized_key
                            .keys
                            .into_iter()
                            .map(|k| PublicOptionDto {
                                content: k.content,
                                image_url: k.image_url,
                            })
                            .collect(),
                    }),
                )
            }
            crate::domain::question::model::QuestionType::MultipleChoice => {
                let deserialized_key: crate::domain::question::model::OptionKeys =
                    serde_json::from_value(value.key.clone()).unwrap();
                (
                    "multipleChoice".to_string(),
                    json!(PublicOptionsDto {
                        keys: deserialized_key
                            .keys
                            .into_iter()
                            .map(|k| PublicOptionDto {
                                content: k.content,
                                image_url: k.image_url,
                            })
                            .collect(),
                    }),
                )
            }
            crate::domain::question::model::QuestionType::TextEntry => (
                "textEntry".to_string(),
                json!(PublicTextDto {
                    placeholder: "Your answer here".to_string(),
                }),
            ),
        };
        PublicQuestionDto {
            id: value.id,
            r#type,
            content: value.content,
            image_url: value.image_url,
            public_data,
            quiz_id: value.quiz_id,
            created_at: value.created_at.map(|dt| dt.to_rfc3339()),
        }
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct CreateQuestionParamsDto {
    #[schema(example = 1)]
    pub quiz_id: i32,

    #[schema(example = "singleChoice")]
    pub r#type: String,

    #[schema(example = "What is the capital of France?")]
    pub content: String,

    #[schema(example = "https://example.com/image.png")]
    pub image_url: Option<String>,

    #[schema(value_type = Object)]
    #[schema(example = json!({
        "keys": [
            {
                "content": "Paris",
                "image_url": null,
                "is_correct": true,
                "explanation": "Paris is the capital of France."
            },
            {
                "content": "London",
                "image_url": null,
                "is_correct": false,
                "explanation": "London is the capital of the UK."
            }
        ]
    }))]
    pub key: Value,
}

impl From<CreateQuestionParamsDto> for CreateQuestionParams {
    fn from(value: CreateQuestionParamsDto) -> Self {
        CreateQuestionParams {
            quiz_id: value.quiz_id,
            r#type: match value.r#type.as_str() {
                "singleChoice" => crate::domain::question::model::QuestionType::SingleChoice,
                "multipleChoice" => crate::domain::question::model::QuestionType::MultipleChoice,
                "textEntry" => crate::domain::question::model::QuestionType::TextEntry,
                _ => panic!("Invalid question type"),
            },
            content: value.content,
            image_url: value.image_url,
            key: value.key,
        }
    }
}

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct UpdateQuestionParamsDto {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "singleChoice")]
    pub r#type: Option<String>,

    #[schema(example = "What is the capital of France?")]
    pub content: Option<String>,

    #[schema(example = "https://example.com/image.png")]
    pub image_url: Option<String>,

    #[schema(value_type = Object)]
    #[schema(example = json!({
        "keys": [
            {
                "content": "Paris",
                "image_url": null,
                "is_correct": true,
                "explanation": "Paris is the capital of France."
            },
            {
                "content": "London",
                "image_url": null,
                "is_correct": false,
                "explanation": "London is the capital of the UK."
            }
        ]
    }))]
    pub key: Option<Value>,
}

impl From<UpdateQuestionParamsDto> for UpdateQuestionParams {
    fn from(value: UpdateQuestionParamsDto) -> Self {
        UpdateQuestionParams {
            id: value.id,
            r#type: value.r#type.map(|t| match t.as_str() {
                "singleChoice" => crate::domain::question::model::QuestionType::SingleChoice,
                "multipleChoice" => crate::domain::question::model::QuestionType::MultipleChoice,
                "textEntry" => crate::domain::question::model::QuestionType::TextEntry,
                _ => panic!("Invalid question type"),
            }),
            content: value.content,
            image_url: value.image_url,
            key: value.key,
        }
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
// pub struct KeyOptionDto {
//     pub content: String,
//     pub image_url: Option<String>,
//     pub is_correct: bool,
//     pub explanation: Option<String>,
// }

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PublicOptionDto {
    pub content: String,
    pub image_url: Option<String>,
}

// #[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
// pub struct OptionKeysDto {
//     pub keys: Vec<KeyOptionDto>,
// }

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PublicOptionsDto {
    pub keys: Vec<PublicOptionDto>,
}

// #[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
// pub struct TextKeyDto {
//     pub correct_entry: String,
//     pub explanation: Option<String>,
// }

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PublicTextDto {
    pub placeholder: String,
}
