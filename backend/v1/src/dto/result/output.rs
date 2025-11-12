use serde::Serialize;

use crate::{
    dto::{
        question::output::QuestionWithKeyDto, quiz::output::QuizMinimalDto, result::UserAnswerDto,
    },
    models::result::{QuestionResult, QuizResult, QuizResultSummary},
};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestionResultDto {
    pub question_with_key: QuestionWithKeyDto,
    pub user_answer: UserAnswerDto,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizResultSummaryDto {
    pub id: i32,
    pub user_id: i32,
    pub quiz: QuizMinimalDto,
    pub score: f64,
    pub total_questions: i32,
    pub correct_answers: i32,
    // pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QuizResultDto {
    pub summary: QuizResultSummaryDto,
    pub result: Vec<QuestionResultDto>,
}

impl Into<QuestionResultDto> for QuestionResult {
    fn into(self) -> QuestionResultDto {
        QuestionResultDto {
            question_with_key: self.question_with_key.into(),
            user_answer: self.user_answer.into(),
        }
    }
}

impl Into<QuizResultSummaryDto> for QuizResultSummary {
    fn into(self) -> QuizResultSummaryDto {
        QuizResultSummaryDto {
            id: self.id,
            user_id: self.user_id,
            quiz: self.quiz.into(),
            score: self.score,
            total_questions: self.total_questions,
            correct_answers: self.correct_answers,
        }
    }
}

impl Into<QuizResultDto> for QuizResult {
    fn into(self) -> QuizResultDto {
        let mut result = Vec::new();
        for r in self.result {
            result.push(r.into());
        }
        QuizResultDto {
            summary: self.summary.into(),
            result,
        }
    }
}
