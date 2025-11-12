use serde::Deserialize;

use crate::models::result::paginate::{QuestionResultQuery, QuizResultSummaryQuery};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuizResultSummaryQueryDto {
    pub user_id: i32,
    pub page: i64,
    pub size: i64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct QuestionResultQueryDto {
    pub result_id: i32,
    pub page: i64,
    pub size: i64,
}

impl From<QuizResultSummaryQueryDto> for QuizResultSummaryQuery {
    fn from(value: QuizResultSummaryQueryDto) -> Self {
        Self {
            user_id: value.user_id,
            page: value.page,
            size: value.size,
        }
    }
}

impl From<QuestionResultQueryDto> for QuestionResultQuery {
    fn from(value: QuestionResultQueryDto) -> Self {
        Self {
            result_id: value.result_id,
            page: value.page,
            size: value.size,
        }
    }
}
