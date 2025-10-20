use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::{json, Value};

use crate::{
    database::pool::QuizBankPool,
    models::{
        result::paginate::QuestionAnswerResultQuery,
        user::{User, UserMinimal},
    },
};

pub async fn get_my_info(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    let mut connection = match pool.get_connection().await {
        Ok(connection) => connection,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

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

    match User::get_by_id(user_id, &mut *connection).await {
        Ok(user) => Ok(Json(json!(user))),
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn get_my_result(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn get_user_info(
    State(pool): State<QuizBankPool>,
    Path(id): Path<String>,
) -> Result<Json<Value>, StatusCode> {
    let mut connection = match pool.get_connection().await {
        Ok(connection) => connection,
        Err(_) => {
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let id: i32 = id.parse().unwrap_or(-1);
    match UserMinimal::get_by_id(id, &mut *connection).await {
        Ok(user) => Ok(Json(json!(user))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_user_results(
    State(pool): State<QuizBankPool>,
    Query(query): Query<QuestionAnswerResultQuery>,
) -> Result<Json<Value>, StatusCode> {
    todo!()
}
