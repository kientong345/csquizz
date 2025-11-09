use std::sync::Arc;

use axum::{
    Json,
    extract::{Query, State},
};
use serde_json::{Value, json};
use tokio::sync::RwLock;

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        category::{Category, paginate::CategoryQuery},
        pagination::Paginate,
    },
};

pub async fn paginate(
    State(state): State<Arc<RwLock<AppState>>>,
    Query(query): Query<CategoryQuery>,
) -> Result<Json<Value>, ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.get_connection().await?;

    let page = Category::page(&query, &mut *connection).await?;

    Ok(Json(json!(page)))
}
