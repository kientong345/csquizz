use serde::Deserialize;

use crate::models::answer::UnevaluatedAnswer;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizSubmissionParamsDto {
    pub answers_params: Vec<UnevaluatedAnswer>,
}

impl QuizSubmissionParamsDto {
    pub fn bind(self, user_id: i32, quiz_id: i32) -> QuizSubmissionParams {
        QuizSubmissionParams {
            user_id,
            quiz_id,
            answers_params: self.answers_params,
        }
    }
}

/// send this struct to Evaluator
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizSubmissionParams {
    pub user_id: i32,
    pub quiz_id: i32,
    pub answers_params: Vec<UnevaluatedAnswer>,
}
