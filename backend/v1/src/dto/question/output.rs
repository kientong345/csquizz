use serde::Serialize;

use crate::{
    dto::question::{KeyTypeDto, NoKeyTypeDto},
    models::question::{QuestionForm, QuestionNoKey, QuestionWithKey},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionNoKeyDto {
    pub id: i32,
    pub form: String,
    pub text: String,
    pub image_url: Option<String>,
    pub answer_no_key: NoKeyTypeDto,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionWithKeyDto {
    pub id: i32,
    pub form: String,
    pub text: String,
    pub image_url: Option<String>,
    pub answer_key: KeyTypeDto,
}

impl Into<QuestionNoKeyDto> for QuestionNoKey {
    fn into(self) -> QuestionNoKeyDto {
        let form = match self.form {
            QuestionForm::SingleChoice => String::from("single-choice"),
            QuestionForm::MultipleChoice => String::from("multiple-choice"),
            QuestionForm::TextEntry => String::from("text-entry"),
        };

        QuestionNoKeyDto {
            id: self.id,
            form,
            text: self.text,
            image_url: self.image_url,
            answer_no_key: self.answer_no_key.into(),
        }
    }
}

impl Into<QuestionWithKeyDto> for QuestionWithKey {
    fn into(self) -> QuestionWithKeyDto {
        let form = match self.form {
            QuestionForm::SingleChoice => String::from("single-choice"),
            QuestionForm::MultipleChoice => String::from("multiple-choice"),
            QuestionForm::TextEntry => String::from("text-entry"),
        };

        QuestionWithKeyDto {
            id: self.id,
            form,
            text: self.text,
            image_url: self.image_url,
            answer_key: self.answer_key.into(),
        }
    }
}
