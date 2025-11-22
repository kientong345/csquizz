use axum::{Json, extract::State};
use serde_json::Value;

use crate::{app::AppState, controller::error::ControllerError};

pub async fn paginate_me(State(state): State<AppState>) -> Result<Json<Value>, ControllerError> {
    todo!()
}

pub async fn paginate(State(state): State<AppState>) -> Result<Json<Value>, ControllerError> {
    todo!()
}
