use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    quiz::{QuizDifficulty, QuizMetadata},
};

impl QuizMetadata {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuizMetadata, ModelError> {
        Ok(sqlx::query_as!(
            QuizMetadata,
            r#"SELECT
                q.id, q.title, q.description, c.name AS category,
                COALESCE(COUNT(qs.quiz_id), 0) AS "question_count!",
                q.difficulty AS "difficulty: QuizDifficulty", u.display_name AS created_by
            FROM quizzes AS q
                JOIN categories AS c ON q.category = c.id
                JOIN users AS u ON q.created_by = u.id
                LEFT JOIN questions AS qs ON q.id = qs.quiz_id
            WHERE q.id = $1
            GROUP BY q.id, c.name, u.display_name"#,
            id
        )
        .fetch_one(connection)
        .await?)
    }
}

#[cfg(feature = "local")]
#[cfg(test)]
mod tests {
    use sqlx::{pool::PoolConnection, Postgres};

    use crate::{
        database::load_sample,
        models::quiz::{QuizDifficulty, QuizMetadata},
    };

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_quiz_by_id(mut conn: PoolConnection<Postgres>) {
        load_sample(&mut conn).await;

        let quiz1 = QuizMetadata::get_by_id(1, &mut conn).await.unwrap();
        let quiz2 = QuizMetadata::get_by_id(2, &mut conn).await.unwrap();
        let quiz3 = QuizMetadata::get_by_id(3, &mut conn).await.unwrap();

        assert_eq!(quiz1.question_count, 3);
        assert_eq!(quiz2.difficulty.unwrap(), QuizDifficulty::Easy);
        assert_eq!(quiz3.created_by.unwrap(), "bocchi_the_dev".to_string());
    }
}
