use crate::models::{
    comment::{CommentDetail, CommentPaginateParams},
    error::ModelError,
    pagination::{Page, Paginate},
};

impl Paginate<CommentPaginateParams> for CommentDetail {
    async fn page(
        params: &CommentPaginateParams,
        connection: &mut sqlx::PgConnection,
    ) -> Result<Page<Self>, ModelError> {
        todo!()
    }
}
