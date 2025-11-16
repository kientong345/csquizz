use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, Query, State},
    response::IntoResponse,
};
use reqwest::StatusCode;

use crate::features::{
    category::dto::{CategoryQueryDto, CreateCategoryDto, UpdateCategoryDto},
    shared::app_state::AppState,
};

pub async fn get_category_page(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<CategoryQueryDto>,
) -> impl IntoResponse {
    (StatusCode::OK, "Category Page").into_response()
}

pub async fn create_category(
    State(app_state): State<Arc<AppState>>,
    Json(dto): Json<CreateCategoryDto>,
) -> impl IntoResponse {
    // match app_state
    //     .services
    //     .category_service
    //     .create_category(dto.name, dto.image_url, dto.description)
    //     .await
    // {
    //     Ok(category) => (StatusCode::CREATED, Json(category)).into_response(),
    //     Err(e) => e.into_response(),
    // }
}

pub async fn update_category(
    State(app_state): State<Arc<AppState>>,
    Path(category_id): Path<i32>,
    Json(dto): Json<UpdateCategoryDto>,
) -> impl IntoResponse {
    // match app_state
    //     .services
    //     .category_service
    //     .update_category(category_id, dto.name, dto.image_url, dto.description)
    //     .await
    // {
    //     Ok(category) => (StatusCode::OK, Json(category)).into_response(),
    //     Err(e) => e.into_response(),
    // }
}

pub async fn delete_category(
    State(app_state): State<Arc<AppState>>,
    Path(category_id): Path<i32>,
) -> impl IntoResponse {
    // match app_state
    //     .services
    //     .category_service
    //     .delete_category(category_id)
    //     .await
    // {
    //     Ok(_) => (StatusCode::NO_CONTENT).into_response(),
    //     Err(e) => e.into_response(),
    // }
}
