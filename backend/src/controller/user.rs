use axum::{
    Extension, Json,
    extract::{Path, Query, State},
};
use reqwest::StatusCode;
use serde_json::{Value, json};

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        auth::AccessClaims,
        input_dto::user::UserUpdateParamsDto,
        pagination::Paginate,
        user::{DatabaseUser, UserFullDetail, UserPaginateParams, UserPublicDetail},
    },
};

pub async fn get_me(
    State(state): State<AppState>,
    Extension(access_claims): Extension<AccessClaims>,
) -> Result<Json<Value>, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);

    let mut connection = state.primary_db.get_connection().await?;

    // let auth_header = headers
    //     .get("Authorization")
    //     .and_then(|v| v.to_str().ok())
    //     .unwrap_or("");

    // if !auth_header.starts_with("Bearer ") {
    //     return Err(StatusCode::UNAUTHORIZED);
    // }

    // let access_token = auth_header.trim_start_matches("Bearer ").trim();

    // let user_id = match validate_access_token(access_token, &config::secret_key()) {
    //     Ok(user_id) => user_id,
    //     Err(_) => return Err(StatusCode::UNAUTHORIZED),
    // };

    let user = UserFullDetail::get_by_id(user_id, &mut *connection).await?;

    Ok(Json(json!(user)))
}

pub async fn update_me(
    State(state): State<AppState>,
    Extension(access_claims): Extension<AccessClaims>,
    Json(payload): Json<UserUpdateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let user_id = access_claims.sub.parse().unwrap_or(-1);

    let mut connection = state.primary_db.start_transaction().await?;

    DatabaseUser::update_by(&payload.bind(user_id), &mut *connection).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn find_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    let user: UserPublicDetail = UserFullDetail::get_by_id(id, &mut *connection)
        .await?
        .into();
    Ok(Json(json!(user)))
}

pub async fn get_page(
    State(state): State<AppState>,
    Query(query): Query<UserPaginateParams>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    let users = UserPublicDetail::page(&query, &mut *connection).await?;

    Ok(Json(json!(users)))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UserUpdateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    DatabaseUser::update_by(&payload.bind(id), &mut *connection).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    DatabaseUser::delete_by(id, &mut *connection).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}
