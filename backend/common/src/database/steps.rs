use std::iter;

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

    pub async fn update_step(
        &self,
        step_id: &StepId,
        params: &serde_json::Value,
    ) -> Result<(), ServiceError> {
        sqlx::query!(
            // language=PostgreSQL
            r#"
                UPDATE "steps" SET step_parameters = $1 WHERE step_id = $2;
            "#,
            params,
            step_id.inner(),
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_step(&self, step_id: &StepId) -> Result<(), ServiceError> {
        let id = step_id.inner();
        sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE FROM steps WHERE step_id = $1;
        "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn connect_steps(&self, from: Step, to: Vec<StepId>) -> Result<(), ServiceError> {
        let from_ids: Vec<_> = iter::repeat_n(from.step_id.inner().clone(), to.len()).collect();
        let pipeline_ids: Vec<_> =
            iter::repeat_n(from.pipeline_id.into_inner(), to.len()).collect();
        let to_ids: Vec<_> = to.into_iter().map(|v| v.inner().clone()).collect();
        sqlx::query!(
        // language=PostgreSQL
        r#"
            INSERT INTO "step_connections" (from_step_id, to_step_id, pipeline_id) SELECT * FROM UNNEST($1::uuid[], $2::uuid[], $3::uuid[])
        "#,
        from_ids.as_slice(),
        to_ids.as_slice(),
        pipeline_ids.as_slice()
    )
    .fetch_one(&self.pool)
    .await?;
        Ok(())
    }

    pub async fn disconnect_steps(
        &self,
        from: &StepId,
        to: Vec<StepId>,
    ) -> Result<(), ServiceError> {
        let to: Vec<_> = to.into_iter().map(|v| v.into_inner()).collect();
        sqlx::query!(
            // language=PostgreSQL
            r#"
            DELETE FROM "step_connections" WHERE from_step_id = $1 AND to_step_id = ANY($2)
        "#,
            from.inner(),
            to.as_slice(),
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn get_step_connections(
        &self,
        step_id: &StepId,
    ) -> Result<Vec<StepId>, ServiceError> {
        let step_ids = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT to_step_id FROM "step_connections" WHERE from_step_id = $1
        "#,
            step_id.inner(),
        )
        .map(|v| StepId::from(v.to_step_id))
        .fetch_all(&self.pool)
        .await?;
        Ok(step_ids)
    }
}
