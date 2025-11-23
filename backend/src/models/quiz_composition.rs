use serde::Serialize;

use crate::models::{
    comment::CommentDetail,
    error::ModelError,
    pagination::Page,
    question::{QuestionPrivateData, QuestionPublicData},
    quiz::QuizDetail,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizPublicQuestion {
    pub metadata: QuizDetail,
    pub data: Page<QuestionPublicData>,
}

impl TryFrom<QuizPrivateQuestion> for QuizPublicQuestion {
    type Error = ModelError;
    fn try_from(value: QuizPrivateQuestion) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: value.metadata,
            data: value.data.try_map_into()?,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizPrivateQuestion {
    pub metadata: QuizDetail,
    pub data: Page<QuestionPrivateData>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizComment {
    pub metadata: QuizDetail,
    pub data: Page<CommentDetail>,
}
