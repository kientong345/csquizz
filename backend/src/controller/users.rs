use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde_json::{json, Value};
use tokio::sync::RwLock;

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        result::paginate::QuestionResultQuery,
        user::{UserFullDetail, UserPubInfo},
    },
};

pub async fn get_my_info(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<Json<Value>, ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.get_connection().await?;

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

    let user_id = -1;

    let user = UserFullDetail::get_by_id(user_id, &mut *connection).await?;

    Ok(Json(json!(user)))
}

pub async fn get_my_result(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<Json<Value>, ControllerError> {
    todo!()
}

pub async fn get_user_by_id(
    State(state): State<Arc<RwLock<AppState>>>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.get_connection().await?;

    let id: i32 = id.parse().unwrap_or(-1);
    let user = UserPubInfo::get_by_id(id, &mut *connection).await?;
    Ok(Json(json!(user)))
}

pub async fn get_user_results(
    State(state): State<Arc<RwLock<AppState>>>,
    Query(query): Query<QuestionResultQuery>,
) -> Result<Json<Value>, ControllerError> {
    todo!()
}
