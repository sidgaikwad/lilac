use crate::{
    model::{
        pipeline::PipelineId,
        step::{StepInstance, StepInstanceId},
    },
    ServiceError,
};

use super::Database;

impl Database {
    pub async fn get_step_instance(
        &self,
        step_instance_id: &StepInstanceId,
    ) -> Result<StepInstance, ServiceError> {
        let id = step_instance_id.inner();
        let step_instance = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT *
            FROM "step_instances"
            WHERE step_instance_id = $1
        "#,
            id
        )
        .map(|row| StepInstance {
            step_instance_id: row.step_instance_id.into(),
            step_id: row.step_id.into(),
            pipeline_id: row.pipeline_id.into(),
            previous_step: row.previous_step.map(|v| v.into()),
            next_step: row.next_step.map(|v| v.into()),
            step_parameters: row.step_parameters.into(),
        })
        .fetch_one(&self.pool)
        .await?;
        Ok(step_instance)
    }

    pub async fn list_step_instances(
        &self,
        pipeline_id: &PipelineId,
    ) -> Result<Vec<StepInstance>, ServiceError> {
        let id = pipeline_id.inner();
        let step_instances = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT *
            FROM "step_instances"
            WHERE pipeline_id = $1
        "#,
            id
        )
        .map(|row| StepInstance {
            step_instance_id: row.step_instance_id.into(),
            step_id: row.step_id.into(),
            pipeline_id: row.pipeline_id.into(),
            previous_step: row.previous_step.map(|v| v.into()),
            next_step: row.next_step.map(|v| v.into()),
            step_parameters: row.step_parameters.into(),
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(step_instances)
    }

    pub async fn create_step_instance(
        &self,
        step_instance: StepInstance,
    ) -> Result<StepInstanceId, ServiceError> {
        let step_instance_id = sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "step_instances" (step_instance_id, step_id, pipeline_id, next_step, previous_step, step_parameters) VALUES ($1, $2, $3, $4, $5, $6) RETURNING step_instance_id
        "#,
        step_instance.step_instance_id.inner(),
        step_instance.step_id.inner(),
        step_instance.pipeline_id.inner(),
        step_instance.next_step.as_ref().map(|v| v.inner()),
        step_instance.previous_step.as_ref().map(|v| v.inner()),
        &step_instance.step_parameters,
    )
    .map(|row| StepInstanceId::from(row.step_instance_id))
    .fetch_one(&self.pool)
    .await?;
        Ok(step_instance_id)
    }
}
