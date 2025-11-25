use serde_json::{Value, json};
use sqlx::PgConnection;

use crate::{
    models::{
        input_dto::{
            submission_result::SubmissionResultPaginateParamsDto, user::UserUpdateParamsDto,
        },
        pagination::Paginate,
        submission_result::SubmissionResultMinimal,
        user::{DatabaseUser, UserFullDetail, UserPaginateParams, UserPublicDetail},
    },
    services::error::ServiceError,
};

#[derive(Clone)]
pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self
    }

    pub async fn get_me(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
    ) -> Result<Value, ServiceError> {
        let user = UserFullDetail::get_by_id(user_id, conn).await?;
        Ok(json!(user))
    }

    pub async fn update_me(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        payload: &UserUpdateParamsDto,
    ) -> Result<(), ServiceError> {
        DatabaseUser::update_by(&payload.clone().bind(user_id), conn).await?;
        Ok(())
    }

    pub async fn find_user_by_id(
        &self,
        conn: &mut PgConnection,
        id: i32,
    ) -> Result<Value, ServiceError> {
        let user: UserPublicDetail = UserFullDetail::get_by_id(id, conn).await?.into();
        Ok(json!(user))
    }

    pub async fn get_users_page(
        &self,
        conn: &mut PgConnection,
        query: &UserPaginateParams,
    ) -> Result<Value, ServiceError> {
        let users = UserPublicDetail::page(query, conn).await?;
        Ok(json!(users))
    }

    pub async fn update_user(
        &self,
        conn: &mut PgConnection,
        id: i32,
        payload: &UserUpdateParamsDto,
    ) -> Result<(), ServiceError> {
        DatabaseUser::update_by(&payload.clone().bind(id), conn).await?;
        Ok(())
    }

    pub async fn delete_user(&self, conn: &mut PgConnection, id: i32) -> Result<(), ServiceError> {
        DatabaseUser::delete_by(id, conn).await?;
        Ok(())
    }

    pub async fn get_submissions_me(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        params: &SubmissionResultPaginateParamsDto,
    ) -> Result<Value, ServiceError> {
        let submissions =
            SubmissionResultMinimal::page(&params.clone().bind(user_id), conn).await?;
        Ok(json!(submissions))
    }
}
