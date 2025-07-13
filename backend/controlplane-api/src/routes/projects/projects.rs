use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use validator::Validate;

use crate::{
    auth::claims::Claims,
    model::{
        integration::AWSIntegration,
        project::{Project, ProjectId},
    },
    ServiceError,
};

use crate::AppState;

#[instrument(level = "info", skip(app_state), ret, err)]
pub async fn create_project(
    claims: Claims,
    State(app_state): State<AppState>,
    Json(request): Json<CreateProjectRequest>,
) -> Result<Json<CreateProjectResponse>, ServiceError> {
    let db = &app_state.db;

    match request.validate() {
        Ok(_) => (),
        Err(e) => return Err(ServiceError::BadRequest(e.to_string())),
    }
    let project = Project::create(request.name);
    let project_id = project.project_id.clone();

    db.create_project_with_membership(project, &claims.sub)
        .await?;

    Ok(Json(CreateProjectResponse { id: project_id }))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateProjectRequest {
    #[validate(length(min = 1, message = "Project name cannot be empty"))]
    name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateProjectResponse {
    id: ProjectId,
}

#[instrument(level = "info", skip(app_state), ret, err)]
pub async fn get_project(
    claims: Claims,
    State(app_state): State<AppState>,
    Path(project_id): Path<String>,
) -> Result<Json<GetProjectResponse>, ServiceError> {
    let db = &app_state.db;
    let project_id = ProjectId::try_from(project_id)?;
    let is_member = db.is_user_project_member(&claims.sub, &project_id).await?;

    if !is_member {
        return Err(ServiceError::Unauthorized {
            reason: String::from("user is not a member of project"),
        });
    }

    let project = db.get_project(&project_id).await?;

    Ok(Json(project.into()))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetProjectResponse {
    id: ProjectId,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    aws_integration: Option<AWSIntegration>,
}

impl From<Project> for GetProjectResponse {
    fn from(project: Project) -> Self {
        Self {
            id: project.project_id,
            name: project.project_name,
            aws_integration: project.aws_integration,
        }
    }
}

#[instrument(level = "info", skip(app_state), ret, err)]
pub async fn delete_project(
    claims: Claims,
    State(app_state): State<AppState>,
    Path(project_id_str): Path<String>,
) -> Result<(), ServiceError> {
    let db = &app_state.db;
    let project_id = ProjectId::try_from(project_id_str)?;

    let is_member = db.is_user_project_member(&claims.sub, &project_id).await?;

    if !is_member {
        return Err(ServiceError::Unauthorized {
            reason: String::from("user is not a member of project"),
        });
    }

    db.delete_project(&project_id).await?;

    Ok(())
}

#[instrument(level = "info", skip(app_state), ret, err)]
pub async fn list_projects(
    claims: Claims,
    State(app_state): State<AppState>,
) -> Result<Json<ListProjectsResponse>, ServiceError> {
    let db = &app_state.db;
    let projects = db
        .list_projects_for_user(&claims.sub)
        .await?
        .into_iter()
        .map(|project| project.into())
        .collect();

    Ok(Json(ListProjectsResponse { projects }))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListProjectsResponse {
    projects: Vec<GetProjectResponse>,
}
