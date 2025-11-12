use serde::Deserialize;

use crate::models::category::{paginate::CategoryQuery, post::PostCategory};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct PostCategoryDto {
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CategoryQueryDto {
    pub name_pattern: Option<String>,
    pub page: i64,
    pub size: i64,
}

impl From<PostCategoryDto> for PostCategory {
    fn from(value: PostCategoryDto) -> Self {
        Self {
            name: value.name,
            image_url: value.image_url,
            description: value.description,
        }
    }
}

impl From<CategoryQueryDto> for CategoryQuery {
    fn from(value: CategoryQueryDto) -> Self {
        Self {
            name_pattern: value.name_pattern,
            page: value.page,
            size: value.size,
        }
    }
}
