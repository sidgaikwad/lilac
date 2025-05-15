use axum::{extract::State, Json};
use common::{database::Database, model::step_definition::StepDefinition, ServiceError};
use serde::Serialize;
use tracing::instrument;

use crate::auth::claims::Claims;

#[instrument(skip(db))]
pub async fn list_step_definitions(
    claims: Claims,
    State(db): State<Database>,
) -> Result<Json<ListStepDefinitionsResponse>, ServiceError> {
    let step_definitions = db.list_step_definitions().await?.into_iter().collect();

    Ok(Json(ListStepDefinitionsResponse { step_definitions }))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListStepDefinitionsResponse {
    step_definitions: Vec<StepDefinition>,
}