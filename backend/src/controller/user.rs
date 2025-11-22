use axum::{
    Json,
    extract::{Path, State},
};
use serde_json::{Value, json};

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::user::{UserFullDetail, UserPublicDetail},
};

pub async fn get_me(State(state): State<AppState>) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.pool.get_connection().await?;

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

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.pool.get_connection().await?;

    let id: i32 = id.parse().unwrap_or(-1);
    let user: UserPublicDetail = UserFullDetail::get_by_id(id, &mut *connection)
        .await?
        .into();
    Ok(Json(json!(user)))
}
