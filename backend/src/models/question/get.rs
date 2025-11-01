use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    question::{
        KeyType, NoKeyType, OptionContent, OptionKey, QuestionForm, QuestionNoKey, QuestionWithKey,
        TextKey,
    },
};

impl QuestionNoKey {
    pub async fn get_by_id(
        question_id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuestionNoKey, ModelError> {
        let row = sqlx::query!(
            r#"SELECT id, question_type AS "form: QuestionForm", question_text AS text, image_url, answer_key
            FROM questions WHERE id = $1"#,
            question_id,
        ).fetch_one(connection).await?;

        let answer_no_key = match row.form {
            QuestionForm::MultipleChoice => {
                let option_keys: Vec<OptionKey> = serde_json::from_value(row.answer_key)?;
                let mut option_contents = Vec::new();
                for key in option_keys {
                    option_contents.push(OptionContent(key.content));
                }
                NoKeyType::MultipleChoiceKey(option_contents)
            }
            QuestionForm::SingleChoice => {
                let option_keys: Vec<OptionKey> = serde_json::from_value(row.answer_key)?;
                let mut option_contents = Vec::new();
                for key in option_keys {
                    option_contents.push(OptionContent(key.content));
                }
                NoKeyType::SingleChoiceKey(option_contents)
            }
            QuestionForm::TextEntry => {
                let _text_key: TextKey = serde_json::from_value(row.answer_key)?;
                NoKeyType::TextEntryKey
            }
        };

        Ok(QuestionNoKey {
            id: row.id,
            form: row.form,
            text: row.text,
            image_url: row.image_url,
            answer_no_key,
        })
    }
}

impl QuestionWithKey {
    pub async fn get_by_id(
        question_id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuestionWithKey, ModelError> {
        let row = sqlx::query!(
            r#"SELECT id, question_type AS "form: QuestionForm", question_text AS text, image_url, answer_key
            FROM questions WHERE id = $1"#,
            question_id,
        ).fetch_one(connection).await?;

        let answer_key = match row.form {
            QuestionForm::MultipleChoice => {
                let option_keys: Vec<OptionKey> = serde_json::from_value(row.answer_key)?;
                KeyType::MultipleChoiceKey(option_keys)
            }
            QuestionForm::SingleChoice => {
                let option_keys: Vec<OptionKey> = serde_json::from_value(row.answer_key)?;
                KeyType::SingleChoiceKey(option_keys)
            }
            QuestionForm::TextEntry => {
                let text_key: TextKey = serde_json::from_value(row.answer_key)?;
                KeyType::TextEntryKey(text_key)
            }
        };

        Ok(QuestionWithKey {
            id: row.id,
            form: row.form,
            text: row.text,
            image_url: row.image_url,
            answer_key,
        })
    }
}
