use std::str::FromStr;

use sqlx::PgConnection;

use crate::models::{
    error::ModelError,
    pagination::{Page, Paginate},
    user::{UserPaginateParams, UserPublicDetail, UserSortField},
};

impl Paginate<UserPaginateParams> for UserPublicDetail {
    /// Get a paginated list of users
    async fn page(
        params: &UserPaginateParams,
        connection: &mut PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        let offset = (params.page.saturating_sub(1)) * params.page_size;

        let order = match UserSortField::from_str(params.sort_by.as_str())? {
            UserSortField::QuizCompletedCount => "quiz_completed_count",
            UserSortField::QuizCreatedCount => "quiz_created_count",
            UserSortField::FollowerCount => "follower_count",
        };

        let items = sqlx::query_as!(
            UserPublicDetail,
            r#"SELECT
                u.usr_id AS id, u.usr_display_name AS display_name,
                u.usr_avatar_url AS avatar_url, u.usr_created_at AS "created_at: _",
                COALESCE(sub_cnt.quiz_completed_count, 0) AS "quiz_completed_count!",
                COALESCE(created_cnt.quiz_created_count, 0) AS "quiz_created_count!",
                COALESCE(fl_cnt.follower_count, 0) AS "follower_count!"
            FROM users AS u
            INNER JOIN (
                SELECT sub_user_id, COUNT(*) AS quiz_completed_count
                FROM submission_results
                WHERE sub_is_passed = TRUE
                GROUP BY sub_user_id
            ) AS sub_cnt
            ON u.usr_id = sub_cnt.sub_user_id
            INNER JOIN (
                SELECT qz_creator_id, COUNT(*) AS quiz_created_count
                FROM quizzes
                GROUP BY qz_creator_id
            ) AS created_cnt
            ON u.usr_id = created_cnt.qz_creator_id
            INNER JOIN (
                SELECT ufl_followed_id, COUNT(*) AS follower_count
                FROM user_followers AS fl
                GROUP BY ufl_followed_id
            ) AS fl_cnt
            ON u.usr_id = fl_cnt.ufl_followed_id
            WHERE u.usr_display_name ILIKE $1
            ORDER BY $2 DESC
            OFFSET $3 LIMIT $4"#,
            format!("%{}%", params.name_pattern),
            order,
            offset as i64,
            params.page_size as i64,
        )
        .fetch_all(&mut *connection)
        .await?;

        let total_items = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM users WHERE usr_display_name ILIKE $1"#,
            format!("%{}%", params.name_pattern),
        )
        .fetch_one(connection)
        .await?
        .unwrap_or(0);

        Ok(Page::build_from(items, total_items, params.page_size))
    }
}
