use serde::Deserialize;
use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    question::{post::PostQuestion, Question},
    quiz::{post::PostQuizMetadata, QuizMetadata},
    result::{
        AnswerResultType, QuestionAnswerResult, QuestionContent, QuizResult, QuizResultSummary,
    },
};

#[derive(Debug, Deserialize)]
pub enum AnswerType {
    ChoicesAnswer(Vec<i32>), // Vec<option_id>
    TextAnswer(String),
}

#[derive(Debug, Deserialize)]
pub struct SubmittedAnswer {
    pub question_id: i32,
    pub answer: AnswerType,
}

#[derive(Debug, Deserialize)]
pub struct Submission {
    pub user_id: i32,
    pub quiz_id: i32,
    pub answers: Vec<SubmittedAnswer>,
}

impl Submission {
    pub async fn evaluate(self, connection: &mut PgConnection) -> Result<QuizResult, ModelError> {
        let mut result = Vec::new();
        let total_questions = self.answers.len() as i32;
        let mut correct_answers = 0;

        for SubmittedAnswer {
            question_id,
            answer,
        } in self.answers
        {
            match answer {
                AnswerType::ChoicesAnswer(choices) => {
                    let question =
                        QuestionContent::from(Question::get_by_id(question_id, connection).await?);
                    let mut answer_results = Vec::new();
                    for choice in choices {
                        let is_correct = sqlx::query!(
                            r#"SELECT is_correct FROM options WHERE id = $1"#,
                            choice,
                        )
                        .fetch_one(&mut *connection)
                        .await?
                        .is_correct
                        .unwrap_or(false);
                        answer_results.push((choice, is_correct));
                    }

                    result.push(QuestionAnswerResult {
                        question_id,
                        question,
                        answer: AnswerResultType::ChoicesResult(answer_results),
                    });
                }
                AnswerType::TextAnswer(text_entry) => {
                    let question =
                        QuestionContent::from(Question::get_by_id(question_id, connection).await?);
                    let correct_entry = sqlx::query!(
                        r#"SELECT correct_entry FROM questions WHERE id = $1"#,
                        question_id,
                    )
                    .fetch_one(&mut *connection)
                    .await?
                    .correct_entry
                    .unwrap_or_default();
                    let is_correct = &text_entry == &correct_entry;
                    if is_correct {
                        correct_answers += 1;
                    }

                    result.push(QuestionAnswerResult {
                        question_id,
                        question,
                        answer: AnswerResultType::TextResult(text_entry, is_correct),
                    });
                }
            }
        }

        let quiz_info = QuizMetadata::get_by_id(self.quiz_id, connection).await?;
        Ok(QuizResult {
            summary: QuizResultSummary {
                id: -1,
                quiz_id: Some(quiz_info.id),
                quiz_title: quiz_info.title,
                score: (correct_answers / total_questions) as f64,
                total_questions,
                correct_answers,
            },
            result,
        })
    }
}

#[derive(Debug, Deserialize)]
pub struct PostQuiz {
    pub metadata: PostQuizMetadata,
    pub questions: Vec<PostQuestion>,
}
