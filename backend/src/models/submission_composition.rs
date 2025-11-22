use serde::Deserialize;
use serde_json::Value;

use crate::models::{
    answer::AnswerCreateParams, question::QuestionCreateParams, quiz::QuizCreateParams,
};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RawQuestionCreateParams {
    pub r#type: String,
    pub content: String,
    pub image_url: Option<String>,
    pub key: Value,
}

impl RawQuestionCreateParams {
    pub fn into_full(self, quiz_id: i32) -> QuestionCreateParams {
        QuestionCreateParams {
            quiz_id,
            r#type: self.r#type,
            content: self.content,
            image_url: self.image_url,
            key: self.key,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuizQuestionCreateParams {
    pub quiz_params: QuizCreateParams,
    pub questions_params: Vec<RawQuestionCreateParams>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct RawAnswerCreateParams {
    pub question_id: i32,
    pub data: Value,
}

impl RawAnswerCreateParams {
    pub fn into_full(self, result_id: i32, is_correct: bool) -> AnswerCreateParams {
        AnswerCreateParams {
            result_id,
            question_id: self.question_id,
            is_correct,
            data: self.data,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuizSubmissionParams {
    pub quiz_id: i32,
    pub answers_params: Vec<RawAnswerCreateParams>,
}
