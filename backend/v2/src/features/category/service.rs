use std::sync::Arc;

use crate::{
    domain::category::{
        model::{CreateCategoryParams, UpdateCategoryParams},
        repository::ICategoryRepository,
    },
    features::category::{dto::CategoryDto, error::CategoryResult},
};

pub struct CategoryService {
    repository: Arc<dyn ICategoryRepository>,
}

impl CategoryService {
    pub fn build_from(repository: Arc<dyn ICategoryRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_category(
        &self,
        name: String,
        image_url: Option<String>,
        description: Option<String>,
    ) -> CategoryResult<CategoryDto> {
        // let params = CreateCategoryParams {
        //     name,
        //     image_url,
        //     description,
        // };
        // let category = self.category_repository.create(&params).await?;

        // // The created category doesn't have the quiz_count, so we return a DTO with a default value.
        // // For a more accurate count, a refetch would be needed, but 0 is acceptable for a new category.
        // Ok(CategoryDto {
        //     id: category.id,
        //     name: category.name,
        //     image_url: category.image_url,
        //     description: category.description,
        //     quiz_count: 0,
        // })
        todo!()
    }

    pub async fn list_categories(&self) -> CategoryResult<Vec<CategoryDto>> {
        // let categories_with_counts = self.category_repository.list_with_counts().await?;
        // let category_dtos = categories_with_counts
        //     .into_iter()
        //     .map(|c| CategoryDto {
        //         id: c.id,
        //         name: c.name,
        //         image_url: c.image_url,
        //         description: c.description,
        //         quiz_count: c.quiz_count,
        //     })
        //     .collect();
        // Ok(category_dtos)
        todo!()
    }

    pub async fn update_category(
        &self,
        category_id: i32,
        name: Option<String>,
        image_url: Option<String>,
        description: Option<String>,
    ) -> CategoryResult<CategoryDto> {
        // let params = UpdateCategoryParams {
        //     name,
        //     image_url,
        //     description,
        // };
        // self.category_repository
        //     .update(category_id, &params)
        //     .await?;

        // // Refetch with count to return the updated DTO
        // let categories = self.category_repository.list_with_counts().await?;
        // let updated_category = categories
        //     .into_iter()
        //     .find(|c| c.id == category_id)
        //     .unwrap(); // Should exist

        // Ok(CategoryDto {
        //     id: updated_category.id,
        //     name: updated_category.name,
        //     image_url: updated_category.image_url,
        //     description: updated_category.description,
        //     quiz_count: updated_category.quiz_count,
        // })
        todo!()
    }

    pub async fn delete_category(&self, category_id: i32) -> CategoryResult<()> {
        // self.repository.delete(category_id).await?;
        // Ok(())
        todo!()
    }
}
