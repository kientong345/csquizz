use axum::{
    Extension, Json,
    extract::{Path, Query, State},
};
use reqwest::StatusCode;
use serde_json::Value;

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        auth::AccessClaims,
        input_dto::{
            submission_result::SubmissionResultPaginateParamsDto, user::UserUpdateParamsDto,
        },
        user::UserPaginateParams,
    },
};

pub async fn get_me(
    State(state): State<AppState>,
    Extension(access_claims): Extension<AccessClaims>,
) -> Result<Json<Value>, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);

    let mut connection = state.primary_db.start_transaction().await?;

    let user = state.user_service.get_me(&mut *connection, user_id).await?;

    connection.commit().await?;

    Ok(Json(user))
}

pub async fn update_me(
    State(state): State<AppState>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<UserUpdateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);

    let mut connection = state.primary_db.start_transaction().await?;

    state
        .user_service
        .update_me(&mut *connection, user_id, &payload)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn find_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    let user = state
        .user_service
        .find_user_by_id(&mut *connection, id)
        .await?;

    connection.commit().await?;

    Ok(Json(user))
}

pub async fn get_users_page(
    State(state): State<AppState>,
    Query(query): Query<UserPaginateParams>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    let users = state
        .user_service
        .get_users_page(&mut *connection, &query)
        .await?;

    connection.commit().await?;

    Ok(Json(users))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UserUpdateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    state
        .user_service
        .update_user(&mut *connection, id, &payload)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    state.user_service.delete_user(&mut *connection, id).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn get_submissions_me(
    State(state): State<AppState>,
    Extension(access_claims): Extension<AccessClaims>,
    Query(params): Query<SubmissionResultPaginateParamsDto>,
) -> Result<Json<Value>, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);

    let mut connection = state.primary_db.start_transaction().await?;

    let submissions = state
        .user_service
        .get_submissions_me(&mut *connection, user_id, &params)
        .await?;

    connection.commit().await?;

    Ok(Json(submissions))
}
