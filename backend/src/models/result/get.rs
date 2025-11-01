use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    question::{QuestionForm, QuestionWithKey},
    quiz::{QuizDifficulty, QuizMinimal},
    result::{
        FetchedQuizSummary, QuestionResult, QuizResult, QuizResultSummary, UserAnswer, UserChoice,
        UserEntry,
    },
    submission::{EvaluatedQuestionResult, EvaluatedQuizResultSummary},
};

impl QuizResultSummary {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuizResultSummary, ModelError> {
        let fetched_quiz_summary = sqlx::query_as!(
            FetchedQuizSummary,
            r#"SELECT
                r.id, r.user_id AS "user_id!", r.quiz_id AS "quiz_id!", q.title AS quiz_title, c.name AS quiz_category,
                q.difficulty AS "quiz_difficulty: QuizDifficulty", r.score, r.total_questions, r.correct_answers
            FROM results AS r JOIN quizzes AS q ON r.quiz_id = q.id JOIN categories AS c ON q.category = c.id WHERE r.id = $1"#,
            id
        )
        .fetch_one(&mut *connection)
        .await?;

        Ok(QuizResultSummary {
            id: fetched_quiz_summary.id,
            user_id: fetched_quiz_summary.user_id,
            quiz: QuizMinimal {
                id: fetched_quiz_summary.quiz_id,
                title: fetched_quiz_summary.quiz_title,
                category: fetched_quiz_summary.quiz_category,
                difficulty: fetched_quiz_summary.quiz_difficulty,
            },
            score: fetched_quiz_summary.score,
            total_questions: fetched_quiz_summary.total_questions,
            correct_answers: fetched_quiz_summary.correct_answers,
        })
    }
}

impl QuestionResult {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuestionResult, ModelError> {
        let row = sqlx::query!(
            r#"SELECT question_id, answer_data FROM user_answers WHERE id = $1"#,
            id
        )
        .fetch_one(&mut *connection)
        .await?;

        let question_with_key =
            QuestionWithKey::get_by_id(row.question_id.unwrap_or(-1), connection).await?;

        let user_answer = match question_with_key.form {
            QuestionForm::MultipleChoice => {
                let choices: Vec<UserChoice> = serde_json::from_value(row.answer_data)?;
                UserAnswer::MultipleChoiceAnswer(choices)
            }
            QuestionForm::SingleChoice => {
                let choice: UserChoice = serde_json::from_value(row.answer_data)?;
                UserAnswer::SingleChoiceAnswer(choice)
            }
            QuestionForm::TextEntry => {
                let entry: UserEntry = serde_json::from_value(row.answer_data)?;
                UserAnswer::TextEntryAnswer(entry)
            }
        };

        Ok(QuestionResult {
            question_with_key,
            user_answer,
        })
    }
}

impl QuizResult {
    pub async fn get_from(
        summary_data: &EvaluatedQuizResultSummary,
        result_data: &Vec<EvaluatedQuestionResult>,
        connection: &mut PgConnection,
    ) -> Result<QuizResult, ModelError> {
        todo!()
    }
}
