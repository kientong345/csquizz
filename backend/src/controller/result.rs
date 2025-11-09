use std::sync::Arc;

use axum::{Json, extract::State};
use serde_json::Value;
use tokio::sync::RwLock;

use crate::{app::AppState, controller::error::ControllerError};

pub async fn paginate_me(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<Json<Value>, ControllerError> {
    todo!()
}

pub async fn paginate(
    State(state): State<Arc<RwLock<AppState>>>,
) -> Result<Json<Value>, ControllerError> {
    todo!()
}
