use serde::Deserialize;
use sqlx::PgConnection;

use crate::models::question::Question;

#[derive(Debug, Deserialize)]
pub struct PostOption {
    pub text: String,
    pub is_correct: bool,
}

#[derive(Debug, Deserialize)]
pub enum PostQuestionForm {
    MultipleChoice(Vec<PostOption>),
    SingleChoice(Vec<PostOption>),
    TextEntry(String), // correct_entry
}

#[derive(Debug, Deserialize)]
pub struct PostQuestion {
    pub text: String,
    pub image_url: Option<String>,
    pub explanation: Option<String>,
    pub form: PostQuestionForm,
}

impl Question {
    pub async fn create(
        quiz_id: i32,
        data: Vec<PostQuestion>,
        connection: &mut PgConnection,
    ) -> Result<Vec<Question>, sqlx::Error> {
        // let (form, options_to_insert, correct_entry) = match data.form {
        //     PostQuestionForm::MultipleChoice(options) => {
        //         (QuestionForm::MultipleChoice, Some(options), None)
        //     }
        //     PostQuestionForm::SingleChoice(options) => {
        //         (QuestionForm::SingleChoice, Some(options), None)
        //     }
        //     PostQuestionForm::TextEntry(correct_text) => {
        //         (QuestionForm::TextEntry, None, Some(correct_text))
        //     }
        // };

        // let question_id = sqlx::query!(
        //     r#"
        //     INSERT INTO questions (quiz_id, question_type, question_text, explanation, correct_entry, image_url)
        //     VALUES ($1, $2, $3, $4, $5, $6)
        //     RETURNING id
        //     "#,
        //     quiz_id,
        //     form.to_string() as _,
        //     data.text,
        //     data.explanation,
        //     correct_entry,
        //     data.image_url
        // )
        // .fetch_one(&mut *tx)
        // .await?
        // .id;

        // if let Some(options) = options_to_insert {
        //     for option in options {
        //         sqlx::query!(
        //             r#"
        //             INSERT INTO options (question_id, option_text, is_correct)
        //             VALUES ($1, $2, $3)
        //             "#,
        //             question_id,
        //             option.text,
        //             option.is_correct
        //         )
        //         .execute(&mut *tx)
        //         .await?;
        //     }
        // }

        // tx.commit().await?;

        // // Fetch the newly created question with all its details
        // question::Question::get_by_id(question_id, connection).await

        todo!()
    }
}
