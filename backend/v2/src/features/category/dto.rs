use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::domain::category::model::{
    Category, CategoryQuery, CreateCategoryParams, UpdateCategoryParams,
};

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDto {
    pub id: i32,

    #[schema(example = "Data Structures")]
    pub name: String,

    #[schema(example = "https://example.com/category_icon.png")]
    pub image_url: Option<String>,

    #[schema(example = "Quizzes related to fundamental data structures.")]
    pub description: Option<String>,
}

impl From<Category> for CategoryDto {
    fn from(model: Category) -> Self {
        Self {
            id: model.id,
            name: model.name,
            image_url: model.image_url,
            description: model.description,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename = "snake_case")]
pub struct CreateCategoryDto {
    #[schema(example = "Data Structures")]
    pub name: String,

    #[schema(example = "https://example.com/category_icon.png")]
    pub image_url: Option<String>,

    #[schema(example = "Quizzes related to fundamental data structures.")]
    pub description: Option<String>,
}

impl From<CreateCategoryDto> for CreateCategoryParams {
    fn from(dto: CreateCategoryDto) -> Self {
        Self {
            name: dto.name,
            image_url: dto.image_url,
            description: dto.description,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename = "snake_case")]
pub struct UpdateCategoryDto {
    #[schema(example = 1)]
    pub id: i32,

    #[schema(example = "Algorithms")]
    pub name: Option<String>,

    #[schema(example = "https://example.com/new_category_icon.png")]
    pub image_url: Option<String>,

    #[schema(example = "Quizzes related to fundamental algorithms.")]
    pub description: Option<String>,
}

impl From<UpdateCategoryDto> for UpdateCategoryParams {
    fn from(dto: UpdateCategoryDto) -> Self {
        Self {
            id: dto.id,
            name: dto.name,
            image_url: dto.image_url,
            description: dto.description,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename = "snake_case")]
pub struct CategoryQueryDto {
    #[schema(example = "Data")]
    pub name_pattern: Option<String>,

    #[schema(example = 1)]
    pub page: i32,

    #[schema(example = 10)]
    pub limit: i32,
}

impl From<CategoryQueryDto> for CategoryQuery {
    fn from(dto: CategoryQueryDto) -> Self {
        Self {
            name_pattern: dto.name_pattern,
            page: dto.page,
            limit: dto.limit,
        }
    }
}
