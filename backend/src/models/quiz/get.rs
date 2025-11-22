use sqlx::PgConnection;

use crate::models::{
    category::Category,
    comment::DatabaseComment,
    error::ModelError,
    like::DatabaseQuizLike,
    question::DatabaseQuestion,
    quiz::{DatabaseQuiz, QuizDetail},
};

impl DatabaseQuiz {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<DatabaseQuiz, ModelError> {
        Ok(sqlx::query_as!(
            DatabaseQuiz,
            r#"SELECT
                qz_id AS id, qz_title AS title, qz_description AS description,
                qz_difficulty AS "difficulty: _", qz_category_id AS category_id, qz_creator_id AS creator_id,
                qz_pass_score AS pass_score, qz_created_at AS created_at, qz_updated_at AS updated_at
            FROM quizzes
            WHERE qz_id = $1"#,
            id
        )
        .fetch_one(connection)
        .await?)
    }

    pub async fn count_by_creator_id(
        user_id: i32,
        connection: &mut PgConnection,
    ) -> Result<i64, ModelError> {
        Ok(sqlx::query_scalar!(
            "SELECT COUNT(*) FROM quizzes WHERE qz_creator_id = $1",
            user_id
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0))
    }
}

impl QuizDetail {
    pub async fn get_by_id(
        id: i32,
        connection: &mut PgConnection,
    ) -> Result<QuizDetail, ModelError> {
        let db_quiz = DatabaseQuiz::get_by_id(id, connection).await?;

        let difficulty = if let Some(diff) = db_quiz.difficulty {
            Some(diff.to_string())
        } else {
            None
        };
        let question_count = DatabaseQuestion::count_by_quiz_id(id, connection).await?;
        let like_count = DatabaseQuizLike::count_by_quiz_id(id, connection).await?;
        let comment_count = DatabaseComment::count_by_quiz_id(id, connection).await?;
        let category_name = if let Some(category_id) = db_quiz.category_id {
            Some(Category::get_by_id(category_id, connection).await?.name)
        } else {
            None
        };

        Ok(QuizDetail {
            id,
            title: db_quiz.title,
            description: db_quiz.description,
            difficulty,
            category_id: db_quiz.category_id,
            creator_id: db_quiz.creator_id,
            pass_score: db_quiz.pass_score,
            created_at: db_quiz.created_at.map(|dt| dt.to_rfc3339()),
            updated_at: db_quiz.updated_at.map(|dt| dt.to_rfc3339()),
            question_count,
            like_count,
            comment_count,
            category_name,
        })
    }
}

#[cfg(feature = "local")]
#[cfg(test)]
mod tests {
    use sqlx::{Postgres, pool::PoolConnection};

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
