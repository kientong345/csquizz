use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Struct đại diện cho một chủ đề (category) của quiz.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    #[serde(rename = "id")]
    pub cat_id: i32,

    #[serde(rename = "name")]
    pub cat_name: String,

    #[serde(rename = "imageUrl")]
    pub cat_image_url: Option<String>,

    #[serde(rename = "description")]
    pub cat_description: Option<String>,
}
