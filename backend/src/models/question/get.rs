use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    question::{DatabaseQuestion, QuestionPrivateData},
};

impl DatabaseQuestion {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<DatabaseQuestion, ModelError> {
        Ok(sqlx::query_as!(
            DatabaseQuestion,
            r#"SELECT
                qs_id AS id, qs_type AS "type: _", qs_content AS content, qs_image_url AS image_url,
                qs_key AS "key: serde_json::Value", qs_quiz_id AS "quiz_id!", qs_created_at AS created_at
            FROM questions
            WHERE qs_id = $1"#,
            id,
        )
        .fetch_one(connection)
        .await?)
    }

    pub async fn count_by_quiz_id(
        quiz_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM questions WHERE qs_quiz_id = $1"#,
            quiz_id
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0))
    }
}

impl QuestionPrivateData {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuestionPrivateData, ModelError> {
        let db_question = DatabaseQuestion::get_by_id(id, connection).await?;

        Ok(QuestionPrivateData {
            id,
            r#type: db_question.r#type.to_string(),
            content: db_question.content,
            image_url: db_question.image_url,
            private_data: db_question.key,
            quiz_id: db_question.quiz_id,
            created_at: db_question.created_at.map(|dt| dt.to_rfc3339()),
        })
    }
}

#[cfg(feature = "local")]
#[cfg(test)]
mod tests {
    use sqlx::{Postgres, pool::PoolConnection};

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
