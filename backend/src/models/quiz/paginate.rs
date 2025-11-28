use std::str::FromStr;

use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    pagination::{Page, Paginate},
    quiz::{QuizDifficulty, QuizMinimal, QuizPaginateParams, QuizSortField},
};

impl Paginate<QuizPaginateParams> for QuizMinimal {
    async fn page(
        params: &QuizPaginateParams,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let offset = (params.page.saturating_sub(1)) * params.page_size;

        let title_pattern = format!(
            "%{}%",
            params.title_pattern.clone().unwrap_or("".to_string())
        );
        let difficulty = if let Some(diff) = &params.difficulty {
            Some(QuizDifficulty::from_str(&diff)?)
        } else {
            None
        };
        let order = match QuizSortField::from_str(params.sort_by.as_str())? {
            QuizSortField::CreatedAt => "quiz_completed_count",
            QuizSortField::LikeCount => "like_count",
        };

        let items = sqlx::query_as!(
            QuizMinimal,
            r#"SELECT
                q.qz_id AS id, q.qz_title AS title, q.qz_difficulty AS "difficulty: _",
                COALESCE(qs_cnt.question_count, 0) AS "question_count!",
                COALESCE(lk_cnt.like_count, 0) AS "like_count!",
                c.cat_name AS category_name
            FROM quizzes AS q
            INNER JOIN (
                SELECT qs_quiz_id, COUNT(*) AS question_count
                FROM questions
                GROUP BY qs_quiz_id
            ) AS qs_cnt
            ON q.qz_id = qs_cnt.qs_quiz_id
            INNER JOIN (
                SELECT qzlk_quiz_id, COUNT(*) AS like_count
                FROM quiz_likes
                GROUP BY qzlk_quiz_id
            ) AS lk_cnt
            ON q.qz_id = lk_cnt.qzlk_quiz_id
            INNER JOIN categories AS c
            ON q.qz_category_id = c.cat_id
            WHERE q.qz_title ILIKE $1 AND q.qz_difficulty = $2
            ORDER BY $3 DESC
            OFFSET $4 LIMIT $5"#,
            title_pattern,
            difficulty.clone() as Option<QuizDifficulty>,
            order,
            offset as i64,
            params.page_size as i64,
        )
        .fetch_all(&mut *connection)
        .await?;

        let total_items = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM quizzes WHERE qz_title ILIKE $1 AND qz_difficulty = $2"#,
            title_pattern,
            difficulty as Option<QuizDifficulty>,
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0);

        Ok(Page::build_from(
            items,
            total_items,
            params.page as i64,
            params.page_size,
        ))
    }
}

#[cfg(feature = "local")]
#[cfg(test)]
mod tests {
    use sqlx::{Postgres, pool::PoolConnection};

    use crate::{
        database::load_sample,
        models::{
            pagination::Paginate,
            quiz::{QuizMetadata, paginate::QuizQuery},
        },
    };

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_quiz_page_no_filter(mut conn: PoolConnection<Postgres>) {
        load_sample(&mut conn).await;

        let quiz_query = QuizQuery {
            category_id: None,
            title_pattern: None,
            difficulty: None,
            created_by: None,
            completed_by: None,
            page: 1,
            size: 10,
        };

        let quiz_page = QuizMetadata::page(&quiz_query, &mut conn).await.unwrap();

        assert_eq!(quiz_page.total_items, 3);
        assert_eq!(quiz_page.total_pages, 1);
        assert_eq!(
            quiz_page.items[0].title,
            "Array and String Basics".to_string()
        );
        assert_eq!(quiz_page.items[1].category, "Algorithms".to_string());
        assert_eq!(quiz_page.items[2].question_count, 2);
    }

    #[sqlx::test(migrations = "./migrations")]
    async fn test_get_quiz_page_with_filter(mut conn: PoolConnection<Postgres>) {
        load_sample(&mut conn).await;

        let quiz_query = QuizQuery {
            category_id: None,
            title_pattern: None,
            difficulty: Some("easy".to_string()),
            created_by: None,
            completed_by: None,
            page: 1,
            size: 10,
        };

        let quiz_page = QuizMetadata::page(&quiz_query, &mut conn).await.unwrap();

        assert_eq!(quiz_page.total_items, 2);
        assert_eq!(quiz_page.total_pages, 1);
        assert_eq!(quiz_page.items[0].category, "Data Structures".to_string());
        assert_eq!(quiz_page.items[1].question_count, 2);
    }
}
