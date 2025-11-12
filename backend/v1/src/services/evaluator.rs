use serde::Deserialize;
use sqlx::PgConnection;

use crate::{models::{question::{KeyType, Question, QuestionWithKey}, result::{QuizResult, TmpQuizResult, UserAnswer, UserChoice, UserEntry}}, services::error::ServiceError};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmittedAnswer {
    pub question_id: i32,
    pub question_form: String, // "single-choice" || "multiple-choice" || "text-entry"
    pub single_choice: Option<i32>,
    pub multiple_choices: Option<Vec<i32>>,
    pub entry: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct SubmittedQuiz {
    pub user_id: Option<i32>,
    pub quiz_id: i32,
    pub answers: Vec<SubmittedAnswer>,
}

pub struct Evaluator {

}

impl Evaluator {
    pub async fn evaluate(
        &self,
        submission: SubmittedQuiz,
        connection: &mut PgConnection,
    ) -> Result<EvaluatedResult, ServiceError> {
        let mut result = Vec::new();
        let total_questions: i32 =
            Question::count_by_quiz_id(submission.quiz_id, connection).await? as i32;
        let mut correct_answers = 0;

        for answer in submission.answers {
            let (question_id, answer) = answer.try_into()?;
            let key = QuestionWithKey::get_by_id(question_id, connection)
                .await?
                .answer_key;
            result.push(EvaluatedQuestionResult {
                question_id,
                answer_data: answer.clone(),
            });

            if is_matched(answer, key) {
                correct_answers += 1;
            }
        }

        let score = if total_questions != 0 {
            (correct_answers as f64 / total_questions as f64) * 100.0
        } else {
            0.00
        };

        Ok(EvaluatedResult {
            user_id: submission.user_id,
            summary: EvaluatedQuizResultSummary {
                quiz_id: submission.quiz_id,
                score,
                total_questions,
                correct_answers,
            },
            result,
        })
    }
}

#[derive(Debug)]
pub struct EvaluatedQuizResultSummary {
    pub quiz_id: i32,
    pub score: f64,
    pub total_questions: i32,
    pub correct_answers: i32,
}

#[derive(Debug)]
pub struct EvaluatedQuestionResult {
    pub question_id: i32,
    pub answer_data: UserAnswer,
}

#[derive(Debug)]
pub struct EvaluatedResult {
    pub user_id: Option<i32>,
    pub summary: EvaluatedQuizResultSummary,
    pub result: Vec<EvaluatedQuestionResult>,
}

impl EvaluatedResult {
    pub async fn into_quiz_result(
        self,
        connection: &mut PgConnection,
    ) -> Result<QuizResult, ServiceError> {
        if self.user_id.is_none() {
            return Err(ServiceError::BadSubmission(
                "unauthorized users won't be able to store their results".to_string(),
            ));
        }
        Ok(QuizResult::create_from(
            self.user_id.unwrap(),
            &self.summary,
            &self.result,
            connection,
        )
        .await?)
    }

    pub async fn into_tmp_quiz_result(
        self,
        connection: &mut PgConnection,
    ) -> Result<TmpQuizResult, ServiceError> {
        Ok(TmpQuizResult::create_from(&self.summary, &self.result, connection).await?)
    }
}

impl TryInto<(i32, UserAnswer)> for SubmittedAnswer {
    type Error = ServiceError;
    fn try_into(self) -> Result<(i32, UserAnswer), Self::Error> {
        match self.question_form.as_str() {
            "single-choice" => {
                if let Some(choice) = self.single_choice {
                    Ok((
                        self.question_id,
                        UserAnswer::SingleChoiceAnswer(UserChoice {
                            option_index: choice,
                        }),
                    ))
                } else {
                    Err(ServiceError::BadSubmission("no single choice provided".to_string()))
                }
            }
            "multiple-choice" => {
                if let Some(choices) = self.multiple_choices {
                    let mut user_choices = Vec::new();
                    for choice in choices {
                        user_choices.push(UserChoice {
                            option_index: choice,
                        });
                    }
                    Ok((
                        self.question_id,
                        UserAnswer::MultipleChoiceAnswer(user_choices),
                    ))
                } else {
                    Err(ServiceError::BadSubmission(
                        "no multiple choice provided".to_string(),
                    ))
                }
            }
            "text-entry" => {
                if let Some(entry) = self.entry {
                    Ok((
                        self.question_id,
                        UserAnswer::TextEntryAnswer(UserEntry {
                            text_entried: entry,
                        }),
                    ))
                } else {
                    Err(ServiceError::BadSubmission("no text entry provided".to_string()))
                }
            }
            _ => Err(ServiceError::BadSubmission("wrong question form".to_string())),
        }
    }
}

fn is_matched(answer: UserAnswer, key: KeyType) -> bool {
    match key {
        KeyType::MultipleChoiceKey(option_keys) => {
            if let UserAnswer::MultipleChoiceAnswer(choices) = answer {
                let mut is_all_correct = true;
                let mut is_any_incorrect = false;
                for (index, option) in option_keys.iter().enumerate() {
                    let is_chosen = choices
                        .iter()
                        .any(|choice| choice.option_index as usize == index);
                    if option.is_correct && !is_chosen {
                        is_all_correct = false;
                    }
                    if !option.is_correct && is_chosen {
                        is_any_incorrect = true;
                    }
                }
                if is_all_correct && !is_any_incorrect {
                    return true;
                }
            }
        }
        KeyType::SingleChoiceKey(option_keys) => {
            if let UserAnswer::SingleChoiceAnswer(choice) = answer {
                if option_keys[choice.option_index as usize].is_correct {
                    return true;
                }
            }
        }
        KeyType::TextEntryKey(text_entry) => {
            if let UserAnswer::TextEntryAnswer(entry) = answer {
                if entry
                    .text_entried
                    .trim()
                    .eq_ignore_ascii_case(&text_entry.correct_entry.trim())
                {
                    return true;
                }
            }
        }
    }

    false
}

