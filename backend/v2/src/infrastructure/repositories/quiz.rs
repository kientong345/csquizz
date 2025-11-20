use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    domain::{
        error::RepositoryResult,
        page::Page,
        quiz::{
            model::{CreateQuizParams, Quiz, QuizDetail, QuizMinimal, QuizQuery, UpdateQuizParams},
            repository::QuizRepository,
        },
    },
    infrastructure::database::postgres_context::DatabasePool,
};

pub struct SqlxQuizRepository {
    pool: Arc<DatabasePool>,
}

impl SqlxQuizRepository {
    pub fn init(pool: Arc<DatabasePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QuizRepository for SqlxQuizRepository {
    async fn create(&self, params: &CreateQuizParams) -> RepositoryResult<Quiz> {
        // let quiz = sqlx::query_as!(
        //     Quiz,
        //     r#"
        //     INSERT INTO quizzes (qz_title, qz_description, qz_difficulty, qz_category_id, qz_creator_id)
        //     VALUES ($1, $2, $3, $4, $5)
        //     RETURNING
        //         qz_id as id,
        //         qz_title as title,
        //         qz_description as description,
        //         qz_difficulty as "difficulty: _",
        //         qz_category_id as category_id,
        //         qz_creator_id as creator_id,
        //         qz_created_at as created_at,
        //         qz_updated_at as updated_at;
        //     "#,
        //     params.title,
        //     params.description,
        //     params.difficulty as _,
        //     params.category_id,
        //     params.creator_id,
        // )
        // .fetch_one(&self.pool)
        // .await?;

        // Ok(quiz)
        todo!()
    }

    async fn find_by_id(&self, quiz_id: i32) -> RepositoryResult<QuizDetail> {
        // let quiz = sqlx::query_as!(
        //     Quiz,
        //     r#"
        //     SELECT
        //         qz_id as id,
        //         qz_title as title,
        //         qz_description as description,
        //         qz_difficulty as "difficulty: _",
        //         qz_category_id as category_id,
        //         qz_creator_id as creator_id,
        //         qz_created_at as created_at,
        //         qz_updated_at as updated_at
        //     FROM quizzes WHERE qz_id = $1
        //     "#,
        //     quiz_id
        // )
        // .fetch_optional(&self.pool)
        // .await?;

        // Ok(quiz)
        todo!()
    }

    async fn find_all(&self, params: &QuizQuery) -> RepositoryResult<Page<QuizMinimal>> {
        // let mut query_builder: QueryBuilder<sqlx::Postgres> = QueryBuilder::new(
        //     r#"
        //     SELECT
        //         q.qz_id,
        //         q.qz_title,
        //         q.qz_difficulty,
        //         c.cat_id as category_id,
        //         c.cat_name as category_name,
        //         (SELECT COUNT(*) FROM questions qs WHERE qs.qs_quiz_id = q.qz_id) as question_count,
        //         (SELECT COUNT(*) FROM quiz_likes ql WHERE ql.qzlk_quiz_id = q.qz_id) as like_count,
        //         (SELECT COUNT(*) FROM comments cm WHERE cm.cmt_quiz_id = q.qz_id) as comment_count
        //     FROM quizzes q
        //     JOIN categories c ON q.qz_category_id = c.cat_id
        //     WHERE 1=1
        //     "#,
        // );

        // if let Some(category_id) = params.category_id {
        //     query_builder.push(" AND q.qz_category_id = ");
        //     query_builder.push_bind(category_id);
        // }

        // if let Some(difficulty) = &params.difficulty {
        //     query_builder.push(" AND q.qz_difficulty = ");
        //     query_builder.push_bind(difficulty.to_string());
        // }

        // if let Some(search) = &params.search_term {
        //     query_builder.push(" AND q.qz_title ILIKE ");
        //     query_builder.push_bind(format!("%{}%", search));
        // }

        // let mut count_builder = query_builder.clone();
        // count_builder.replace_range(
        //     0..count_builder.sql().find("FROM").unwrap_or(0),
        //     "SELECT COUNT(q.qz_id) as total ",
        // );
        // let count_result = count_builder
        //     .build_query_as::<_TotalCount>()
        //     .fetch_one(&self.pool)
        //     .await?;
        // let total_records = count_result.total.unwrap_or(0) as u32;

        // let sort_column = match params.sort_by.unwrap_or(QuizSortField::CreatedAt) {
        //     QuizSortField::LikeCount => "like_count",
        //     QuizSortField::CommentCount => "comment_count",
        //     QuizSortField::QuestionCount => "question_count",
        //     QuizSortField::Title => "q.qz_title",
        //     QuizSortField::CreatedAt => "q.qz_created_at",
        // };

        // let order = match params.order.unwrap_or(SortOrder::Desc) {
        //     SortOrder::Asc => "ASC",
        //     SortOrder::Desc => "DESC",
        // };

        // query_builder.push(format!(" ORDER BY {} {}", sort_column, order));
        // query_builder.push(" LIMIT ");
        // query_builder.push_bind(params.limit as i64);
        // query_builder.push(" OFFSET ");
        // query_builder.push_bind((params.page - 1) * params.limit as i64);

        // let summaries = query_builder
        //     .build_query_as::<_QuizSummaryRaw>()
        //     .fetch_all(&self.pool)
        //     .await?
        //     .into_iter()
        //     .map(|r| QuizSummary {
        //         id: r.qz_id,
        //         title: r.qz_title,
        //         difficulty: r.qz_difficulty,
        //         question_count: r.question_count.unwrap_or(0),
        //         like_count: r.like_count.unwrap_or(0),
        //         comment_count: r.comment_count.unwrap_or(0),
        //         category_id: r.category_id,
        //         category_name: r.category_name,
        //     })
        //     .collect();

        // Ok((summaries, total_records))
        todo!()
    }

    async fn update(&self, quiz_id: i32, params: &UpdateQuizParams) -> RepositoryResult<Quiz> {
        // let quiz = sqlx::query_as!(
        //     Quiz,
        //     r#"
        //     UPDATE quizzes
        //     SET
        //         qz_title = COALESCE($1, qz_title),
        //         qz_description = COALESCE($2, qz_description),
        //         qz_difficulty = COALESCE($3, qz_difficulty),
        //         qz_category_id = COALESCE($4, qz_category_id),
        //         qz_updated_at = NOW()
        //     WHERE qz_id = $5
        //     RETURNING
        //         qz_id as id,
        //         qz_title as title,
        //         qz_description as description,
        //         qz_difficulty as "difficulty: _",
        //         qz_category_id as category_id,
        //         qz_creator_id as creator_id,
        //         qz_created_at as created_at,
        //         qz_updated_at as updated_at;
        //     "#,
        //     params.title,
        //     params.description,
        //     params.difficulty as _,
        //     params.category_id,
        //     quiz_id
        // )
        // .fetch_one(&self.pool)
        // .await?;

        // Ok(quiz)
        todo!()
    }

    async fn delete(&self, quiz_id: i32) -> RepositoryResult<()> {
        // sqlx::query!("DELETE FROM quizzes WHERE qz_id = $1", quiz_id)
        //     .execute(&self.pool)
        //     .await?;
        // Ok(())
        todo!()
    }
}
