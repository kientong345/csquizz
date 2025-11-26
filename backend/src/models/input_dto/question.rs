use serde::Deserialize;
use serde_json::Value;

use crate::models::question::{QuestionCreateParams, QuestionPaginateParams, QuestionUpdateParams};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionPaginateParamsDto {
    pub page: i32,
    pub page_size: i32,
}

impl QuestionPaginateParamsDto {
    pub fn bind(self, quiz_id: i32) -> QuestionPaginateParams {
        QuestionPaginateParams {
            quiz_id,
            page: self.page,
            page_size: self.page_size,
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionCreateParamsDto {
    pub r#type: String,
    pub content: String,
    pub image_url: Option<String>,
    pub key: Value,
}

impl QuestionCreateParamsDto {
    pub fn bind(self, quiz_id: i32) -> QuestionCreateParams {
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
#[serde(rename_all = "camelCase")]
pub struct QuestionUpdateParamsDto {
    pub r#type: Option<String>,
    pub content: Option<String>,
    pub image_url: Option<String>,
    pub key: Option<Value>,
}

impl QuestionUpdateParamsDto {
    pub fn bind(self, id: i32) -> QuestionUpdateParams {
        QuestionUpdateParams {
            id,
            r#type: self.r#type,
            content: self.content,
            image_url: self.image_url,
            key: self.key,
        }
    }
}
