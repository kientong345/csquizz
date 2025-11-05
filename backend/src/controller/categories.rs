use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::{json, Value};
use tokio::sync::RwLock;

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        category::{paginate::CategoryQuery, Category},
        pagination::Paginate,
    },
};

pub async fn get_categories(
    State(state): State<Arc<RwLock<AppState>>>,
    Query(query): Query<CategoryQuery>,
) -> Result<Json<Value>, ControllerError> {
    let state_locked = state.read().await;
    let mut connection = state_locked.pool.get_connection().await?;

    let page = Category::page(&query, &mut *connection).await?;

    Ok(Json(json!(page)))
}
