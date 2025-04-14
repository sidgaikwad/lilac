use crate::{
    model::{
        pipeline::PipelineId,
        step::{Step, StepId},
    },
    ServiceError,
};

use super::Database;

impl Database {
    pub async fn get_step(&self, step_id: &StepId) -> Result<Step, ServiceError> {
        let id = step_id.inner();
        let step_instance = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT *
            FROM "steps"
            WHERE step_id = $1
        "#,
            id
        )
        .map(|row| Step {
            step_id: row.step_id.into(),
            step_definition_id: row.step_definition_id.into(),
            pipeline_id: row.pipeline_id.into(),
            step_parameters: row.step_parameters.into(),
        })
        .fetch_one(&self.pool)
        .await?;
        Ok(step_instance)
    }

    pub async fn list_steps(&self, pipeline_id: &PipelineId) -> Result<Vec<Step>, ServiceError> {
        let id = pipeline_id.inner();
        let steps = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT *
            FROM "steps"
            WHERE pipeline_id = $1
        "#,
            id
        )
        .map(|row| Step {
            step_id: row.step_id.into(),
            step_definition_id: row.step_definition_id.into(),
            pipeline_id: row.pipeline_id.into(),
            step_parameters: row.step_parameters.into(),
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(steps)
    }

    pub async fn create_step(&self, step: Step) -> Result<StepId, ServiceError> {
        let step_instance_id = sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "steps" (step_id, step_definition_id, pipeline_id, step_parameters) VALUES ($1, $2, $3, $4) RETURNING step_id
        "#,
        step.step_id.inner(),
        step.step_definition_id.inner(),
        step.pipeline_id.inner(),
        &step.step_parameters,
    )
    .map(|row| StepId::from(row.step_id))
    .fetch_one(&self.pool)
    .await?;
        Ok(step_instance_id)
    }

    pub async fn connect_steps(&self, from: Step, to: Step) -> Result<(), ServiceError> {
        sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "step_connections" (from_step_id, to_step_id, pipeline_id) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING
        "#,
        from.step_id.inner(),
        to.step_id.inner(),
        from.pipeline_id.inner(),
    )
    .fetch_one(&self.pool)
    .await?;
        Ok(())
    }

    pub async fn disconnect_steps(&self, from: Step, to: Step) -> Result<(), ServiceError> {
        sqlx::query!(
        // language=PostgreSQL
        r#"
            DELETE FROM "step_connections" WHERE from_step_id = $1 AND to_step_id = $2
        "#,
        from.step_id.inner(),
        to.step_id.inner(),
    )
    .fetch_one(&self.pool)
    .await?;
        Ok(())
    }
}
