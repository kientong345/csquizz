#[derive(Debug, Clone)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub current_page: i64,
    pub total_pages: i64,
    pub total_items: i64,
    pub limit: i64,
}
