use axum::{
    Json,
    extract::{Path, Query, State},
};
use reqwest::StatusCode;
use serde_json::Value;

use crate::{
    app::AppState,
    controller::error::ControllerError,
    models::{
        category::{CategoryCreateParams, CategoryPaginateParams},
        input_dto::category::CategoryUpdateParamsDto,
    },
};

pub async fn get_page(
    State(state): State<AppState>,
    Query(query): Query<CategoryPaginateParams>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    let page = state
        .category_service
        .get_page(&mut *connection, &query)
        .await?;

    connection.commit().await?;

    Ok(Json(page))
}

pub async fn find_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    let categories = state
        .category_service
        .find_by_id(&mut *connection, id)
        .await?;

    connection.commit().await?;

    Ok(Json(categories))
}

pub async fn find_all(State(state): State<AppState>) -> Result<Json<Value>, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    let categories = state.category_service.find_all(&mut *connection).await?;

    connection.commit().await?;

    Ok(Json(categories))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CategoryCreateParams>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    state
        .category_service
        .create(&mut *connection, &payload)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::CREATED)
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    state.category_service.delete(&mut *connection, id).await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<CategoryUpdateParamsDto>,
) -> Result<StatusCode, ControllerError> {
    let mut connection = state.primary_db.start_transaction().await?;

    state
        .category_service
        .update(&mut *connection, id, &payload)
        .await?;

    connection.commit().await?;

    Ok(StatusCode::OK)
}
