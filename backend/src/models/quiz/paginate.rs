use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, QueryBuilder};

use crate::models::{
    error::ModelError,
    pagination::{Page, Paginate},
    quiz::QuizMetadata,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct QuizQuery {
    pub category_id: Option<i32>,
    pub title_pattern: Option<String>,
    pub difficulty: Option<String>,
    pub created_by: Option<i32>,
    pub completed_by: Option<i32>,
    pub page: i64,
    pub size: i64,
}

impl QuizQuery {
    fn apply_filters_for(&self, builder: &mut QueryBuilder<sqlx::Postgres>) {
        builder.push(" WHERE 1=1");

        if let Some(category_id) = self.category_id {
            builder.push(" AND q.category = ").push_bind(category_id);
        }

        if let Some(title) = &self.title_pattern {
            builder
                .push(" AND q.title ILIKE ")
                .push_bind(format!("%{}%", title));
        }

        if let Some(difficulty) = &self.difficulty {
            builder
                .push(" AND q.difficulty = (")
                .push_bind(difficulty.clone())
                .push(")::quiz_difficulty");
        }

        if let Some(user_id) = self.created_by {
            builder.push(" AND q.created_by = ").push_bind(user_id);
        }

        if let Some(user_id) = self.completed_by {
            let sub_query =
                " AND EXISTS (SELECT 1 FROM results r WHERE r.quiz_id = q.id AND r.user_id = ";
            builder.push(sub_query).push_bind(user_id).push(")");
        }
    }

    fn apply_pagination_for(&self, builder: &mut QueryBuilder<sqlx::Postgres>) {
        let page_size = self.size;
        let offset = (self.page - 1) * page_size;
        builder.push(" ORDER BY q.id ASC");
        builder.push(" LIMIT ").push_bind(page_size);
        builder.push(" OFFSET ").push_bind(offset);
    }
}

impl Paginate<QuizQuery> for QuizMetadata {
    async fn page(
        query: &QuizQuery,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let mut count_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            "SELECT COUNT(q.id) FROM quizzes AS q JOIN categories AS c ON q.category = c.id",
        );
        query.apply_filters_for(&mut count_builder);

        let mut query_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
            "SELECT
                q.id, q.title, q.description, c.name AS category,
                COALESCE(COUNT(qs.quiz_id), 0) AS question_count,
                q.difficulty, u.display_name AS created_by
            FROM quizzes AS q
                JOIN categories AS c ON q.category = c.id
                JOIN users AS u ON q.created_by = u.id
                LEFT JOIN questions AS qs ON q.id = qs.quiz_id",
        );
        query.apply_filters_for(&mut query_builder);
        query_builder.push(" GROUP BY q.id, c.name, u.display_name");
        query.apply_pagination_for(&mut query_builder);

        let total_items: i64 = count_builder
            .build_query_scalar()
            .fetch_one(&mut *connection)
            .await?;

        let items: Vec<QuizMetadata> = query_builder.build_query_as().fetch_all(connection).await?;

        Ok(Page::build_from(items, total_items, query.size))
    }
}

#[cfg(feature = "local")]
#[cfg(test)]
mod tests {
    use sqlx::{pool::PoolConnection, Postgres};

    use crate::{
        database::load_sample,
        models::{
            pagination::Paginate,
            quiz::{paginate::QuizQuery, QuizMetadata},
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
