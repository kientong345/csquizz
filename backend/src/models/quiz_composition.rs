use serde::Serialize;

use crate::models::{
    error::ModelError,
    pagination::{Page, Paginate},
    question::{QuestionPaginateParams, QuestionPrivateData, QuestionPublicData},
    quiz::QuizDetail,
};

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizPublicComposition {
    metadata: QuizDetail,
    data: Page<QuestionPublicData>,
}

impl TryFrom<QuizPrivateComposition> for QuizPublicComposition {
    type Error = ModelError;
    fn try_from(value: QuizPrivateComposition) -> Result<Self, Self::Error> {
        Ok(Self {
            metadata: value.metadata,
            data: value.data.try_map_into()?,
        })
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizPrivateComposition {
    metadata: QuizDetail,
    data: Page<QuestionPrivateData>,
}

pub type QuizCompositionPaginateParams = QuestionPaginateParams;

impl QuizPrivateComposition {
    pub async fn get_by(
        params: &QuizCompositionPaginateParams,
        connection: &mut sqlx::PgConnection,
    ) -> Result<QuizPrivateComposition, ModelError> {
        let metadata = QuizDetail::get_by_id(params.quiz_id, connection).await?;
        let data = QuestionPrivateData::page(&params, connection).await?;

        Ok(QuizPrivateComposition { metadata, data })
    }
}
