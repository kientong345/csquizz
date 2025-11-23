use serde::Deserialize;

use crate::models::input_dto::{question::QuestionCreateParamsDto, quiz::QuizCreateParamsDto};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuizQuestionCreateParamsDto {
    pub quiz_params: QuizCreateParamsDto,
    pub questions_params: Vec<QuestionCreateParamsDto>,
}
