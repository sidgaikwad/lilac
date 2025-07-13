use sqlx::types::Json;

use crate::{
    database::DatabaseError,
    model::{integration::AWSIntegration, project::ProjectId},
};

use super::Database;

impl Database {
    pub async fn set_project_aws_integration(
        &self,
        project_id: &ProjectId,
        aws_integration: &AWSIntegration,
    ) -> Result<(), DatabaseError> {
        let id = project_id.inner();
        sqlx::query("UPDATE projects SET aws_integration = $2 WHERE project_id = $1")
            .bind(id)
            .bind(Json(aws_integration))
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
