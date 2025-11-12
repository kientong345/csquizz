use serde::Serialize;

use crate::models::category::Category;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDto {
    pub id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

impl Into<CategoryDto> for Category {
    fn into(self) -> CategoryDto {
        CategoryDto {
            id: self.id,
            name: self.name,
            image_url: self.image_url,
            description: self.description,
        }
    }
}
