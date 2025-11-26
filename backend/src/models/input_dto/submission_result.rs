use serde::Deserialize;

use crate::models::submission_result::SubmissionResultPaginateParams;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmissionResultPaginateParamsDto {
    pub quiz_title_pattern: String,
    pub passed_only: bool,
    pub quiz_difficulty: Option<String>,
    pub page: i32,
    pub page_size: i32,
    pub sort_by: String,
}

impl SubmissionResultPaginateParamsDto {
    pub fn bind(self, user_id: i32) -> SubmissionResultPaginateParams {
        SubmissionResultPaginateParams {
            user_id,
            quiz_title_pattern: self.quiz_title_pattern,
            passed_only: self.passed_only,
            quiz_difficulty: self.quiz_difficulty,
            page: self.page,
            page_size: self.page_size,
            sort_by: self.sort_by,
        }
    }
}
