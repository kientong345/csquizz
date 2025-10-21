use sqlx::PgConnection;

use crate::models::{error::ModelError, result::QuizResult};

impl QuizResult {
    pub async fn store(&self, connection: &mut PgConnection) -> Result<(), ModelError> {
        todo!()
    }
}
