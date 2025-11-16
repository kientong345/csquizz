use std::sync::Arc;

use async_trait::async_trait;

use crate::{
    domain::{
        comment::{
            model::{
                Comment, CommentDetail, CommentQuery, CreateCommentParams, UpdateCommentParams,
            },
            repository::ICommentRepository,
        },
        error::RepositoryResult,
        page::Page,
    },
    infrastructure::database::postgres_context::DatabasePool,
};

pub struct CommentRepository {
    pool: Arc<DatabasePool>,
}

impl CommentRepository {
    pub fn init(pool: Arc<DatabasePool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ICommentRepository for CommentRepository {
    async fn create_from(&self, params: &CreateCommentParams) -> RepositoryResult<Comment> {
        // let comment = sqlx::query_as!(
        //     Comment,
        //     r#"
        //     INSERT INTO comments (cmt_quiz_id, cmt_user_id, cmt_content)
        //     VALUES ($1, $2, $3)
        //     RETURNING cmt_id as id, cmt_quiz_id as quiz_id, cmt_user_id as user_id, cmt_content as content, cmt_created_at as created_at;
        //     "#,
        //     params.quiz_id,
        //     params.user_id,
        //     params.content
        // )
        // .fetch_one(&self.pool)
        // .await?;
        // Ok(comment)
        todo!()
    }

    async fn get_by(&self, comment_id: i32) -> RepositoryResult<CommentDetail> {
        // let comment = sqlx::query_as!(
        //     Comment,
        //     r#"
        //     SELECT cmt_id as id, cmt_quiz_id as quiz_id, cmt_user_id as user_id, cmt_content as content, cmt_created_at as created_at
        //     FROM comments
        //     WHERE cmt_id = $1
        //     "#,
        //     comment_id
        // )
        // .fetch_optional(&self.pool)
        // .await?;
        // Ok(comment)
        todo!()
    }

    async fn get_page_by(&self, query: &CommentQuery) -> RepositoryResult<Page<CommentDetail>> {
        // // First, count total records for the given quiz
        // let count_result = sqlx::query!(
        //     "SELECT COUNT(*) as total FROM comments WHERE cmt_quiz_id = $1",
        //     params.quiz_id
        // )
        // .fetch_one(&self.pool)
        // .await?;
        // let total_records = count_result.total.unwrap_or(0) as u32;

        // // Then, fetch the paginated data with details
        // let comments = sqlx::query!(
        //     r#"
        //     SELECT
        //         c.cmt_id as "comment_id!",
        //         c.cmt_quiz_id as "comment_quiz_id!",
        //         c.cmt_user_id as "comment_user_id!",
        //         c.cmt_content as "comment_content!",
        //         c.cmt_created_at as "comment_created_at!",

        //         u.usr_id as "author_id!",
        //         u.usr_display_name as "author_display_name!",
        //         u.usr_email as "author_email!",
        //         u.usr_avatar_url as "author_avatar_url",
        //         u.usr_role as "author_role: _",
        //         u.usr_google_id as "author_google_id",
        //         u.usr_password_hash as "author_password_hash",
        //         u.usr_created_at as "author_created_at",

        //         (SELECT COUNT(*) FROM comment_likes cl WHERE cl.cmtlk_comment_id = c.cmt_id) as "like_count!"
        //     FROM comments c
        //     JOIN users u ON c.cmt_user_id = u.usr_id
        //     WHERE c.cmt_quiz_id = $1
        //     ORDER BY c.cmt_created_at DESC
        //     LIMIT $2
        //     OFFSET $3
        //     "#,
        //     params.quiz_id,
        //     params.limit as i64,
        //     (params.page - 1) * params.limit as i64
        // )
        // .fetch_all(&self.pool)
        // .await?
        // .into_iter()
        // .map(|r| CommentWithDetails {
        //     comment: Comment {
        //         id: r.comment_id,
        //         quiz_id: r.comment_quiz_id,
        //         user_id: r.comment_user_id,
        //         content: r.comment_content,
        //         created_at: r.comment_created_at,
        //     },
        //     author: User {
        //         usr_id: r.author_id,
        //         usr_display_name: r.author_display_name,
        //         usr_email: r.author_email,
        //         usr_avatar_url: r.author_avatar_url,
        //         usr_role: r.author_role,
        //         usr_google_id: r.author_google_id,
        //         usr_password_hash: r.author_password_hash,
        //         usr_created_at: r.author_created_at,
        //     },
        //     like_count: r.like_count.unwrap_or(0),
        // })
        // .collect();

        // Ok((comments, total_records))
        todo!()
    }

    async fn update_by(&self, params: &UpdateCommentParams) -> RepositoryResult<Comment> {
        // let comment = sqlx::query_as!(
        //     Comment,
        //     r#"
        //     UPDATE comments
        //     SET
        //         cmt_content = COALESCE($1, cmt_content)
        //     WHERE cmt_id = $2
        //     RETURNING cmt_id as id, cmt_quiz_id as quiz_id, cmt_user_id as user_id, cmt_content as content, cmt_created_at as created_at;
        //     "#,
        //     params.content,
        //     comment_id
        // )
        // .fetch_one(&self.pool)
        // .await?;

        // Ok(comment)
        todo!()
    }

    async fn delete_by(&self, comment_id: i32) -> RepositoryResult<()> {
        // sqlx::query!("DELETE FROM comments WHERE cmt_id = $1", comment_id)
        //     .execute(&self.pool)
        //     .await?;
        // Ok(())
        todo!()
    }
}
