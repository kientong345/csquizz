use std::str::FromStr;

use crate::models::{
    comment::{CommentDetail, CommentPaginateParams, CommentSortField},
    error::ModelError,
    pagination::{Page, Paginate},
};

impl Paginate<CommentPaginateParams> for CommentDetail {
    async fn page(
        params: &CommentPaginateParams,
        connection: &mut sqlx::PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let offset = (params.page.saturating_sub(1)) * params.page_size;

        let order = match CommentSortField::from_str(params.sort_by.as_str())? {
            CommentSortField::Latest => "quiz_completed_count",
            CommentSortField::MostLike => "quiz_created_count",
        };

        let items = sqlx::query_as!(
            CommentDetail,
            r#"SELECT
                cm.cmt_id AS id, cm.cmt_user_id AS "user_id!", cm.cmt_quiz_id AS "quiz_id!",
                cm.cmt_content AS content, cm.cmt_created_at AS "created_at: _",
                u.usr_display_name AS user_display_name, u.usr_avatar_url AS user_avatar_url,
                COALESCE(lk.like_count, 0) AS "like_count!"
            FROM comments AS cm
            INNER JOIN users AS u
            ON cm.cmt_user_id = u.usr_id
            INNER JOIN (
                SELECT cmlk_comment_id, COUNT(*) AS like_count
                FROM comment_likes
                GROUP BY cmlk_comment_id
            ) AS lk
            ON cm.cmt_id = lk.cmlk_comment_id
            WHERE cm.cmt_quiz_id = $1
            ORDER BY $2
            OFFSET $3 LIMIT $4"#,
            params.quiz_id,
            order,
            offset as i64,
            params.page_size as i64,
        )
        .fetch_all(&mut *connection)
        .await?;

        let total_items = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM comments WHERE cmt_quiz_id = $1"#,
            params.quiz_id,
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
