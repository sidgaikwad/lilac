use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    auth::models::Claims,
    integration::{
        models::CreateAWSIntegrationRequest,
        service::IntegrationService,
    },
    project::models::ProjectId,
    user::models::UserId,
};

use super::super::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route(
        "/projects/{project_id}/integrations/aws",
        post(create_aws_integration),
    )
}

#[derive(Debug, Deserialize)]
struct CreateAWSIntegrationPayload {
    role_arn: String,
}

#[derive(Debug, Serialize)]
struct SetAWSAccessInfoResponse {
    external_id: String,
}

#[axum::debug_handler(state = AppState)]
async fn create_aws_integration(
    State(service): State<Arc<dyn IntegrationService>>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<CreateAWSIntegrationPayload>,
) -> impl IntoResponse {
    let req = CreateAWSIntegrationRequest {
        project_id: ProjectId(project_id),
        role_arn: payload.role_arn,
    };

    let result = service
        .create_aws_integration(&UserId(claims.sub), &req)
        .await;

    match result {
        Ok(integration) => {
            let response = SetAWSAccessInfoResponse {
                external_id: integration.external_id,
            };
            (axum::http::StatusCode::CREATED, Json(response)).into_response()
        }
        Err(_) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR).into_response(),
    }
}