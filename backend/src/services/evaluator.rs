use serde_json::Value;
use sqlx::PgConnection;

use crate::{
    models::{
        answer::{EvaluatedAnswer, UserChoice, UserChoices, UserEntry},
        input_dto::submission::QuizSubmissionParams,
        question::{DatabaseQuestion, DatabaseQuestionAlter, KeyType},
        quiz::DatabaseQuiz,
        submission_result::SubmissionResultCreateParams,
    },
    services::error::ServiceError,
};

pub struct Evaluator;

impl Evaluator {
    pub async fn evaluate(
        submission: &QuizSubmissionParams,
        connection: &mut PgConnection,
    ) -> Result<(SubmissionResultCreateParams, Vec<EvaluatedAnswer>), ServiceError> {
        let total_questions =
            DatabaseQuestion::count_by_quiz_id(submission.quiz_id, connection).await?;
        let pass_score = DatabaseQuiz::get_by_id(submission.quiz_id, connection)
            .await?
            .pass_score;

        let mut answers = Vec::new();
        let mut correct_answers = 0;

        for unevaluated_answer in &submission.answers_params {
            let key = DatabaseQuestionAlter::try_from(
                DatabaseQuestion::get_by_id(unevaluated_answer.question_id, connection).await?,
            )?
            .key;

            let is_correct = is_matched(&unevaluated_answer.data, &key)?;

            answers.push(unevaluated_answer.clone().bind(is_correct));

            if is_correct {
                correct_answers += 1;
            }
        }

        let score = if total_questions != 0 {
            (correct_answers as f64 / total_questions as f64) * 100.0
        } else {
            0.00
        };

        Ok((
            SubmissionResultCreateParams {
                user_id: submission.user_id,
                quiz_id: submission.quiz_id,
                score,
                is_passed: score >= pass_score,
            },
            answers,
        ))
    }
}

fn is_matched(answer: &Value, key: &KeyType) -> Result<bool, ServiceError> {
    match key {
        KeyType::MultipleChoiceKey(option_keys) => {
            let choices = UserChoices::try_from(answer.clone())?;
            let mut is_all_correct = true;
            let mut is_any_incorrect = false;
            for option in option_keys.keys.iter() {
                let is_chosen = choices
                    .choices
                    .iter()
                    .any(|choice| choice.option_id == option.id);
                if option.is_correct && !is_chosen {
                    is_all_correct = false;
                }
                if !option.is_correct && is_chosen {
                    is_any_incorrect = true;
                }
            }
            if is_all_correct && !is_any_incorrect {
                return Ok(true);
            }
        }
        KeyType::SingleChoiceKey(option_keys) => {
            let choice = UserChoice::try_from(answer.clone())?;
            if option_keys
                .keys
                .iter()
                .find(|e| e.id == choice.option_id && e.is_correct)
                .is_some()
            {
                return Ok(true);
            }
        }
        KeyType::TextEntryKey(text_entry) => {
            let entry_trimmed = UserEntry::try_from(answer.clone())?
                .entry
                .trim()
                .to_string();

            if text_entry.is_case_sensitive {
                if entry_trimmed.eq(text_entry.correct_entry.trim()) {
                    return Ok(true);
                }
            } else {
                if entry_trimmed.eq_ignore_ascii_case(text_entry.correct_entry.trim()) {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}
