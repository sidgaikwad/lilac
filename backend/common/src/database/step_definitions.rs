use std::str::FromStr;

use crate::{
    model::step_definition::{StepCategory, StepDefinition, StepDefinitionId, StepType},
    ServiceError,
};

use super::Database;

impl Database {
    pub async fn get_step_definition(
        &self,
        step_definition_id: &StepDefinitionId,
    ) -> Result<StepDefinition, ServiceError> {
        let id = step_definition_id.inner();
        let step_definition: Result<StepDefinition, ServiceError> = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT *
            FROM "step_definitions"
            WHERE step_definition_id = $1
        "#,
            id
        )
        .map(|row| {
            Ok(StepDefinition {
                id: row.step_definition_id.into(),
                name: row.name,
                description: row.description,
                category: StepCategory::from_str(row.category.as_str())
                    .map_err(|e| ServiceError::ParseError(format!("invalid step category: {e}")))?,
                step_type: StepType::from_str(row.step_type.as_str())
                    .map_err(|e| ServiceError::ParseError(format!("invalid step type: {e}")))?,
                schema: row.schema,
                inputs: row.inputs,
                outputs: row.outputs,
            })
        })
        .fetch_one(&self.pool)
        .await?;
        Ok(step_definition?)
    }

    pub async fn get_step_definition_by_type(
        &self,
        step_type: &StepType,
    ) -> Result<StepDefinition, ServiceError> {
        let step_definition: Result<StepDefinition, ServiceError> = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT *
            FROM "step_definitions"
            WHERE step_type = $1
        "#,
            step_type.to_string()
        )
        .map(|row| {
            Ok(StepDefinition {
                id: row.step_definition_id.into(),
                name: row.name,
                description: row.description,
                category: StepCategory::from_str(row.category.as_str())
                    .map_err(|e| ServiceError::ParseError(format!("invalid step category: {e}")))?,
                step_type: StepType::from_str(row.step_type.as_str())
                    .map_err(|e| ServiceError::ParseError(format!("invalid step type: {e}")))?,
                schema: row.schema,
                inputs: row.inputs,
                outputs: row.outputs,
            })
        })
        .fetch_one(&self.pool)
        .await?;
        Ok(step_definition?)
    }

    pub async fn list_step_definitions(&self) -> Result<Vec<StepDefinition>, ServiceError> {
        let step_definitions: Vec<Result<StepDefinition, ServiceError>> = sqlx::query!(
            // language=PostgreSQL
            r#"
            SELECT *
            FROM "step_definitions"
        "#
        )
        .map(|row| {
            Ok(StepDefinition {
                id: row.step_definition_id.into(),
                name: row.name,
                description: row.description,
                category: StepCategory::from_str(row.category.as_str())
                    .map_err(|e| ServiceError::ParseError(format!("invalid step category: {e}")))?,
                step_type: StepType::from_str(row.step_type.as_str())
                    .map_err(|e| ServiceError::ParseError(format!("invalid step type: {e}")))?,
                schema: row.schema,
                inputs: row.inputs,
                outputs: row.outputs,
            })
        })
        .fetch_all(&self.pool)
        .await?;
        Ok(step_definitions
            .into_iter()
            .collect::<Result<Vec<StepDefinition>, ServiceError>>()?)
    }

    pub async fn register_step_definition(
        &self,
        step_definition: StepDefinition,
    ) -> Result<(), ServiceError> {
        let _ = sqlx::query!(
            // language=PostgreSQL
            r#"
                INSERT INTO "step_definitions" (step_definition_id, name, description, category, step_type, schema, inputs, outputs)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                ON CONFLICT(step_definition_id)
                DO UPDATE SET
                    step_type = EXCLUDED.step_type,
                    schema = EXCLUDED.schema;
            "#,
            step_definition.id.inner(),
            &step_definition.name,
            step_definition.description.as_ref(),
            &step_definition.category.to_string(),
            &step_definition.step_type.to_string(),
            &step_definition.schema,
            &step_definition.inputs,
            &step_definition.outputs,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
