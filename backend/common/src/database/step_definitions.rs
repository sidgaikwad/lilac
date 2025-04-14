use std::str::FromStr;

use crate::{
    model::step_definition::{StepDefinition, StepDefinitionId, StepType},
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
                step_definition_id: row.step_definition_id.into(),
                step_type: match row.step_type.as_str() {
                    "noop" => StepType::NoOp,
                    s => Err(ServiceError::ParseError(format!("invalid step type: {s}")))?,
                },
                parameter_definitions: row.parameter_definitions,
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
                step_definition_id: row.step_definition_id.into(),
                step_type: StepType::from_str(row.step_type.as_str()).map_err(|e| ServiceError::ParseError(format!("invalid step type: {e}")))?,
                parameter_definitions: row.parameter_definitions,
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
                INSERT INTO "step_definitions" (step_definition_id, step_type, parameter_definitions) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING
            "#,
            step_definition.step_definition_id.inner(),
            &step_definition.step_type.to_string(),
            &step_definition.parameter_definitions,
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
