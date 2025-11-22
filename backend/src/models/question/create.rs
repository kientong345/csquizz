use std::str::FromStr;

use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    question::{DatabaseQuestion, QuestionCreateParams, QuestionType},
};

// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "snake_case")]
// pub struct PostQuestion {
//     pub quiz_id: i32,
//     pub form: String, // "multiple-choice" || "single-choice" || "text-entry"
//     pub text: String,
//     pub image_url: Option<String>,
//     pub option_keys: Option<Vec<OptionKey>>,
//     pub text_key: Option<TextKey>,
// }

// impl Question {
//     pub async fn create_from(
//         data: PostQuestion,
//         connection: &mut PgConnection,
//     ) -> Result<Question, ModelError> {
//         let question = match data.form.as_str() {
//             "multiple-choice" => {
//                 if data.option_keys.is_none() {
//                     return Err(ModelError::BadPost("no option_keys".to_string()));
//                 }
//                 let row = sqlx::query!(
//                     r#"INSERT INTO questions (quiz_id, question_type, question_text, image_url, answer_key)
//                     VALUES ($1, $2::text::question_form, $3, $4, $5)
//                     RETURNING id, question_type AS "form: QuestionForm", question_text AS text, image_url, answer_key"#,
//                     data.quiz_id,
//                     data.form,
//                     data.text,
//                     data.image_url,
//                     serde_json::json!(data.option_keys.unwrap()),
//                 ).fetch_one(connection).await?;

//                 let option_keys: Vec<OptionKey> = serde_json::from_value(row.answer_key)?;
//                 Question::WithKey(QuestionWithKey {
//                     id: row.id,
//                     form: QuestionForm::MultipleChoice,
//                     text: row.text,
//                     image_url: row.image_url,
//                     answer_key: KeyType::MultipleChoiceKey(option_keys).validate()?,
//                 })
//             }
//             "single-choice" => {
//                 if data.option_keys.is_none() {
//                     return Err(ModelError::BadPost("no option_keys".to_string()));
//                 }
//                 let row = sqlx::query!(
//                     r#"INSERT INTO questions (quiz_id, question_type, question_text, image_url, answer_key)
//                     VALUES ($1, $2::text::question_form, $3, $4, $5)
//                     RETURNING id, question_type AS "form: QuestionForm", question_text AS text, image_url, answer_key"#,
//                     data.quiz_id,
//                     data.form,
//                     data.text,
//                     data.image_url,
//                     serde_json::json!(data.option_keys.unwrap()),
//                 ).fetch_one(connection).await?;

//                 let option_keys: Vec<OptionKey> = serde_json::from_value(row.answer_key)?;
//                 Question::WithKey(QuestionWithKey {
//                     id: row.id,
//                     form: QuestionForm::SingleChoice,
//                     text: row.text,
//                     image_url: row.image_url,
//                     answer_key: KeyType::SingleChoiceKey(option_keys).validate()?,
//                 })
//             }
//             "text-entry" => {
//                 if data.text_key.is_none() {
//                     return Err(ModelError::BadPost("no text_key".to_string()));
//                 }
//                 let row = sqlx::query!(
//                     r#"INSERT INTO questions (quiz_id, question_type, question_text, image_url, answer_key)
//                     VALUES ($1, $2::text::question_form, $3, $4, $5)
//                     RETURNING id, question_type AS "form: QuestionForm", question_text AS text, image_url, answer_key"#,
//                     data.quiz_id,
//                     data.form,
//                     data.text,
//                     data.image_url,
//                     serde_json::json!(data.text_key.unwrap()),
//                 ).fetch_one(connection).await?;

//                 let text_key: TextKey = serde_json::from_value(row.answer_key)?;
//                 Question::WithKey(QuestionWithKey {
//                     id: row.id,
//                     form: QuestionForm::TextEntry,
//                     text: row.text,
//                     image_url: row.image_url,
//                     answer_key: KeyType::TextEntryKey(text_key).validate()?,
//                 })
//             }
//             _ => {
//                 return Err(ModelError::BadPost(
//                     "wrong question form, only accept:
//                     \"multiple-choice\" || \"single-choice\" || \"text-entry\""
//                         .to_string(),
//                 ));
//             }
//         };

//         Ok(question)
//     }
// }

impl DatabaseQuestion {
    pub async fn create_from(
        params: &QuestionCreateParams,
        connection: &mut PgConnection,
    ) -> Result<DatabaseQuestion, ModelError> {
        let question_type = QuestionType::from_str(&params.r#type)?;

        Ok(sqlx::query_as!(
            DatabaseQuestion,
            r#"INSERT INTO questions (qs_quiz_id, qs_type, qs_content, qs_image_url, qs_key)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING
                qs_id AS id, qs_type AS "type: _", qs_content AS content, qs_image_url AS image_url,
                qs_key AS "key: _", qs_quiz_id AS "quiz_id!", qs_created_at AS created_at"#,
            params.quiz_id,
            question_type as QuestionType,
            params.content,
            params.image_url,
            params.key,
        )
        .fetch_one(connection)
        .await?)
    }
}
