use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use common::{
    database::Database,
    model::{
        organization::OrganizationId,
        pipeline::PipelineId,
        project::{Project, ProjectId},
    },
    ServiceError,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;

use crate::auth::claims::Claims;

#[instrument(level = "info", skip(db), ret, err)]
pub async fn create_project(
    claims: Claims,
    db: Extension<Database>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<Json<CreateProjectResponse>, ServiceError> {
    let project = Project::create(request.name, request.organization_id);

    let project_id = db.create_project(project).await?;

    Ok(Json(CreateProjectResponse { id: project_id }))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectRequest {
    name: String,
    organization_id: OrganizationId,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProjectResponse {
    id: ProjectId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn get_project(
    claims: Claims,
    db: Extension<Database>,
    Path(project_id): Path<String>,
) -> Result<Json<GetProjectResponse>, ServiceError> {
    let project_id = ProjectId::try_from(project_id)?;
    let project = db.get_project(&project_id).await?;

    Ok(Json(project.into()))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetProjectResponse {
    id: ProjectId,
    name: String,
    organization_id: OrganizationId,
}

impl From<Project> for GetProjectResponse {
    fn from(project: Project) -> Self {
        Self {
            id: project.project_id,
            name: project.project_name,
            organization_id: project.organization_id,
        }
    }
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn delete_project(
    claims: Claims,
    db: Extension<Database>,
    Path(project_id): Path<String>,
) -> Result<(), ServiceError> {
    let project_id = ProjectId::try_from(project_id)?;
    db.delete_project(&project_id).await?;

    Ok(())
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn list_projects(
    claims: Claims,
    db: Extension<Database>,
    Query(request): Query<ListProjectsRequest>,
) -> Result<Json<ListProjectsResponse>, ServiceError> {
    let projects = if let Some(org_id) = request.organization_id {
        db.list_projects(&org_id)
            .await?
            .into_iter()
            .map(|org| org.into())
            .collect()
    } else {
        db.list_projects_for_user(&claims.sub)
            .await?
            .into_iter()
            .map(|org| org.into())
            .collect()
    };

    Ok(Json(ListProjectsResponse { projects }))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListProjectsRequest {
    organization_id: Option<OrganizationId>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListProjectsResponse {
    projects: Vec<GetProjectResponse>,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn list_project_pipelines(
    claims: Claims,
    db: Extension<Database>,
    Path(project_id): Path<ProjectId>,
) -> Result<Json<ListProjectPipelineResponse>, ServiceError> {
    let pipelines = db.list_pipelines(&project_id).await?;

    let response = pipelines
        .into_iter()
        .map(|v| ProjectPipelineResponse {
            id: v.pipeline_id,
            name: v.pipeline_name,
            description: v.description,
        })
        .collect();
    Ok(Json(ListProjectPipelineResponse {
        pipelines: response,
    }))
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPipelineResponse {
    id: PipelineId,
    name: String,
    description: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ListProjectPipelineResponse {
    pipelines: Vec<ProjectPipelineResponse>,
}
