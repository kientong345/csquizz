use axum::{extract::State, http::StatusCode, Json};
use serde_json::{json, Value};

use crate::{
    database::pool::QuizBankPool,
    models::{
        auth::{Registration, SignupMethod},
        user::User,
    },
};

pub async fn handle_register(
    State(pool): State<QuizBankPool>,
    Json(registration): Json<Registration>,
) -> Result<Json<Value>, StatusCode> {
    let mut connection = match pool.start_transaction().await {
        Ok(connection) => connection,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    if !registration.is_valid() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    if User::is_name_taken(&registration.username, &mut connection)
        .await
        .unwrap_or(true)
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let user = match User::create(SignupMethod::WithPassword(registration), &mut connection).await {
        Ok(user) => user,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    match connection.commit().await {
        Ok(_) => Ok(Json(json!(user))),
        Err(_) => Err(StatusCode::UNAUTHORIZED),
    }
}

pub async fn handle_login(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}

pub async fn get_my_info(State(pool): State<QuizBankPool>) -> Result<Json<Value>, StatusCode> {
    todo!()
}
