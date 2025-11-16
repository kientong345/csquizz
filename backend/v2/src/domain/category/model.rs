use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct CreateCategoryParams {
    pub name: String,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct UpdateCategoryParams {
    pub id: i32,
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, FromRow)]
pub struct CategoryQuery {
    pub name_pattern: Option<String>,
    pub page: i32,
    pub limit: i32,
}
