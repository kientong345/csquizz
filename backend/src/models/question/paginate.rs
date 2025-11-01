use serde::{Deserialize, Serialize};
use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    pagination::{Page, Paginate},
    question::{
        KeyType, NoKeyType, OptionContent, OptionKey, Question, QuestionForm, QuestionNoKey,
        QuestionWithKey, TextKey,
    },
};

#[derive(Debug, Deserialize, Serialize)]
pub struct QuestionQuery {
    pub quiz_id: i32,
    pub page: i64,
    pub size: i64,
}

impl Paginate<QuestionQuery> for QuestionNoKey {
    async fn page(
        query: &QuestionQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let offset = (query.page.saturating_sub(1)) * query.size;
        let rows = sqlx::query!(
            r#"SELECT id, question_type AS "form: QuestionForm", question_text AS text, image_url, answer_key
            FROM questions WHERE quiz_id = $1 LIMIT $2 OFFSET $3"#,
            query.quiz_id,
            query.size,
            offset,
        ).fetch_all(&mut *connection).await?;

        let mut items = Vec::new();
        for row in rows {
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

            items.push(QuestionNoKey {
                id: row.id,
                form: row.form,
                text: row.text,
                image_url: row.image_url,
                answer_no_key,
            })
        }

        let total_items = Question::count_by_quiz_id(query.quiz_id, connection).await?;

        Ok(Page::build_from(items, total_items, query.size))
    }
}

impl Paginate<QuestionQuery> for QuestionWithKey {
    async fn page(
        query: &QuestionQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let offset = (query.page.saturating_sub(1)) * query.size;
        let rows = sqlx::query!(
            r#"SELECT id, question_type AS "form: QuestionForm", question_text AS text, image_url, answer_key
            FROM questions WHERE quiz_id = $1 LIMIT $2 OFFSET $3"#,
            query.quiz_id,
            query.size,
            offset,
        ).fetch_all(&mut *connection).await?;

        let mut items = Vec::new();
        for row in rows {
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

            items.push(QuestionWithKey {
                id: row.id,
                form: row.form,
                text: row.text,
                image_url: row.image_url,
                answer_key,
            })
        }

        let total_items = Question::count_by_quiz_id(query.quiz_id, connection).await?;

        Ok(Page::build_from(items, total_items, query.size))
    }
}
