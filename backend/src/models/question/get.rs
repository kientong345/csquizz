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

#[cfg(test)]
mod tests {
    use sqlx::{pool::PoolConnection, Postgres};

    use crate::{
        database::load_sample,
        models::question::{
            KeyType, NoKeyType, QuestionForm, QuestionNoKey, QuestionWithKey, TextKey,
        },
    };

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_nokey_by_id(mut conn: PoolConnection<Postgres>) {
        load_sample(&mut conn).await;

        let question_id1 = QuestionNoKey::get_by_id(1, &mut conn).await.unwrap();
        let question_id3 = QuestionNoKey::get_by_id(3, &mut conn).await.unwrap();
        let question_id2 = QuestionNoKey::get_by_id(2, &mut conn).await.unwrap();

        assert_eq!(question_id1.form, QuestionForm::SingleChoice);
        let option_3rd = if let NoKeyType::SingleChoiceKey(no_keys) = question_id1.answer_no_key {
            no_keys[2].0.clone()
        } else {
            "".to_string()
        };
        assert_eq!(option_3rd, String::from("O(log n)"));
        assert_eq!(question_id3.form, QuestionForm::TextEntry);
        assert_eq!(question_id3.answer_no_key, NoKeyType::TextEntryKey);
        let option_1st = if let NoKeyType::MultipleChoiceKey(no_keys) = question_id2.answer_no_key {
            no_keys[0].0.clone()
        } else {
            "".to_string()
        };
        assert_eq!(option_1st, String::from("String"));
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_key_by_id(mut conn: PoolConnection<Postgres>) {
        load_sample(&mut conn).await;

        let question_id1 = QuestionWithKey::get_by_id(1, &mut conn).await.unwrap();
        let question_id3 = QuestionWithKey::get_by_id(3, &mut conn).await.unwrap();
        let question_id2 = QuestionWithKey::get_by_id(2, &mut conn).await.unwrap();

        assert_eq!(question_id1.form, QuestionForm::SingleChoice);
        let (option_3rd, is_correct) =
            if let KeyType::SingleChoiceKey(keys) = question_id1.answer_key {
                (keys[2].content.clone(), keys[2].is_correct)
            } else {
                ("".to_string(), false)
            };
        assert_eq!((option_3rd, is_correct), (String::from("O(log n)"), false));
        assert_eq!(question_id3.form, QuestionForm::TextEntry);
        assert_eq!(question_id3.answer_key, KeyType::TextEntryKey(
            TextKey {
                correct_entry: "<vector>".to_string(),
                explanation: Some("The <vector> header is required to use the std::vector class, which provides a dynamic array implementation.".to_string())
            }));
        let (option_2nd, is_correct) =
            if let KeyType::MultipleChoiceKey(keys) = question_id2.answer_key {
                (keys[1].content.clone(), keys[1].is_correct)
            } else {
                ("".to_string(), false)
            };
        assert_eq!(
            (option_2nd, is_correct),
            (String::from("StringBuilder"), true)
        );
    }
}
