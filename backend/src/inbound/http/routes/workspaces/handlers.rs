use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;
use uuid::Uuid;

use super::models;
use crate::{
    domain::{
        auth::models::Claims,
        project::models::ProjectId,
        workspace::{models::CreateWorkspaceRequest, service::WorkspaceService},
    },
    inbound::http::{
        errors::ApiError,
        routes::workspaces::models::{CreateWorkspacePayload, WorkspaceResponse},
    },
};

pub async fn create_workspace_handler(
    State(workspace_service): State<Arc<dyn WorkspaceService>>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
    Json(payload): Json<CreateWorkspacePayload>,
) -> Result<impl IntoResponse, ApiError> {
    let req = CreateWorkspaceRequest {
        name: payload.name,
        project_id: ProjectId::new(project_id),
        cluster_id: payload.cluster_id.into(),
        ide: payload.ide,
        image: payload.image,
        cpu_millicores: payload.cpu_millicores,
        memory_mb: payload.memory_mb,
        gpu: payload.gpu,
    };

    let workspace = workspace_service.create_workspace(req, claims.sub).await?;

    Ok((
        StatusCode::CREATED,
        Json(WorkspaceResponse::from(workspace)),
    ))
}

pub async fn list_workspaces_handler(
    State(workspace_service): State<Arc<dyn WorkspaceService>>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<impl IntoResponse, ApiError> {
    let workspaces = workspace_service
        .list_workspaces(ProjectId::new(project_id), claims.sub)
        .await?;

    let workspace_responses: Vec<WorkspaceResponse> = workspaces
        .into_iter()
        .map(WorkspaceResponse::from)
        .collect();

    Ok((StatusCode::OK, Json(workspace_responses)))
}

pub async fn get_workspace_connection_handler(
    State(workspace_service): State<Arc<dyn WorkspaceService>>,
    claims: Claims,
    Path((_project_id, workspace_id)): Path<(Uuid, Uuid)>,
) -> Result<impl IntoResponse, ApiError> {
    let workspace = workspace_service
        .find_by_id(workspace_id.into(), claims.sub)
        .await?;

    let response = models::ConnectionDetailsResponse {
        url: workspace.url,
        token: workspace.token,
    };

    Ok((StatusCode::OK, Json(response)))
}
