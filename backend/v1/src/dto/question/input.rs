use serde::Deserialize;

use crate::{
    dto::question::KeyTypeDto,
    models::question::{paginate::QuestionQuery, post::PostQuestion},
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuestionQueryDto {
    pub quiz_id: i32,
    pub page: i64,
    pub size: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostQuestionDto {
    pub quiz_id: i32,
    pub form: String, // "multiple-choice" || "single-choice" || "text-entry"
    pub text: String,
    pub image_url: Option<String>,
    pub answer_key: KeyTypeDto,
}

impl From<QuestionQueryDto> for QuestionQuery {
    fn from(value: QuestionQueryDto) -> Self {
        Self {
            quiz_id: value.quiz_id,
            page: value.page,
            size: value.size,
        }
    }
}

impl From<PostQuestionDto> for PostQuestion {
    fn from(value: PostQuestionDto) -> Self {
        let option_keys = match value.answer_key.clone() {
            KeyTypeDto::SingleChoice(keys) => {
                let mut opt_keys = Vec::new();
                for key in keys {
                    opt_keys.push(key.into());
                }
                Some(opt_keys)
            }
            KeyTypeDto::MultipleChoice(keys) => {
                let mut opt_keys = Vec::new();
                for key in keys {
                    opt_keys.push(key.into());
                }
                Some(opt_keys)
            }
            _ => None,
        };
        let text_key = match value.answer_key {
            KeyTypeDto::TextEntry(key) => Some(key.into()),
            _ => None,
        };
        Self {
            quiz_id: value.quiz_id,
            form: value.form,
            text: value.text,
            image_url: value.image_url,
            option_keys,
            text_key,
        }
    }
}
