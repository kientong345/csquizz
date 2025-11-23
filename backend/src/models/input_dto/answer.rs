use serde::Deserialize;
use serde_json::Value;

use crate::models::answer::AnswerCreateParams;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct AnswerCreateParamsDto {
    pub question_id: i32,
    pub data: Value,
}

impl AnswerCreateParamsDto {
    pub fn bind(self, result_id: i32, is_correct: bool) -> AnswerCreateParams {
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
pub struct QuizSubmissionParamsDto {
    pub answers_params: Vec<AnswerCreateParamsDto>,
}

impl QuizSubmissionParamsDto {
    pub fn bind(self, quiz_id: i32) -> QuizSubmissionParams {
        QuizSubmissionParams {
            quiz_id,
            answers_params: self.answers_params,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuizSubmissionParams {
    pub quiz_id: i32,
    pub answers_params: Vec<AnswerCreateParamsDto>,
}
