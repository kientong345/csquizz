use serde::{Deserialize, Serialize};
use crate::domain::models::category::Category;

// --- Request DTOs ---

#[derive(Debug, Deserialize)]
pub struct CreateCategoryDto {
    pub name: String,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCategoryDto {
    pub name: Option<String>,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    pub description: Option<String>,
}

// --- Response DTOs ---

#[derive(Debug, Serialize)]
pub struct CategoryDto {
    pub id: i32,
    pub name: String,
    #[serde(rename = "imageUrl")]
    pub image_url: Option<String>,
    pub description: Option<String>,
}

impl From<Category> for CategoryDto {
    fn from(cat: Category) -> Self {
        Self {
            id: cat.cat_id,
            name: cat.cat_name,
            image_url: cat.cat_image_url,
            description: cat.cat_description,
        }
    }
}
