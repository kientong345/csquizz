use serde::Deserialize;

use crate::models::category::CategoryUpdateParams;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct CategoryUpdateParamsDto {
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

impl CategoryUpdateParamsDto {
    pub fn bind(self, id: i32) -> CategoryUpdateParams {
        CategoryUpdateParams {
            id,
            name: self.name,
            image_url: self.image_url,
            description: self.description,
        }
    }
}
