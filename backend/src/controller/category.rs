use axum::{
    Json,
    extract::{Path, Query, State},
};
use reqwest::StatusCode;
use serde_json::{Value, json};

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        category::{Category, CategoryCreateParams, CategoryPaginateParams, CategoryUpdateParams},
        pagination::Paginate,
    },
};

pub async fn get_page(
    State(state): State<AppState>,
    Query(query): Query<CategoryPaginateParams>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    let page = Category::page(&query, &mut *connection).await?;

    Ok(Json(json!(page)))
}

pub async fn find_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    let categories = Category::get_by_id(id, &mut *connection).await?;

    Ok(Json(json!(categories)))
}

pub async fn find_all(State(state): State<AppState>) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    let categories = Category::list_all(&mut *connection).await?;

    Ok(Json(json!(categories)))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CategoryCreateParams>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    Category::create_from(&payload, &mut *connection).await?;

    Ok(StatusCode::CREATED)
}

pub async fn delete(
    State(state): State<AppState>,
    Query(id): Query<i32>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    Category::delete_by(id, &mut *connection).await?;

    Ok(StatusCode::OK)
}

pub async fn update(
    State(state): State<AppState>,
    Json(payload): Json<CategoryUpdateParams>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.get_connection().await?;

    Category::update_by(&payload, &mut *connection).await?;

    Ok(StatusCode::OK)
}
