use std::collections::HashMap;

use sqlx::{prelude::FromRow, PgConnection};

use crate::models::{error::ModelError, question::AnswerOption};

#[derive(Debug, FromRow)]
struct FetchedAnswerOption {
    id: i32,
    option_text: String,
    question_id: i32,
}

impl AnswerOption {
    pub async fn get_by_question_ids(
        question_ids: &[i32],
        connection: &mut PgConnection,
    ) -> Result<HashMap<i32, Vec<AnswerOption>>, ModelError> {
        if question_ids.is_empty() {
            return Ok(HashMap::new());
        }

        let fetched_options = sqlx::query_as::<_, FetchedAnswerOption>(
            "SELECT id, option_text, question_id FROM options WHERE question_id = ANY($1)",
        )
        .bind(question_ids)
        .fetch_all(connection)
        .await?;

        let mut options_map: HashMap<i32, Vec<AnswerOption>> = HashMap::new();
        for fetched in fetched_options {
            options_map
                .entry(fetched.question_id)
                .or_default()
                .push(AnswerOption {
                    id: fetched.id,
                    text: fetched.option_text,
                });
        }
        Ok(options_map)
    }
}
