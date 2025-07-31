use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    domain::{
        auth::models::Claims,
        project::{
            models::{CreateProjectRequest, ProjectId},
            service::ProjectService,
        },
    },
    inbound::http::{
        errors::ApiError,
        routes::projects::models::{
            CreateProjectHttpRequest, CreateProjectHttpResponse, GetProjectHttpResponse,
            ListProjectsHttpResponse,
        },
        AppState,
    },
};

#[axum::debug_handler(state = AppState)]
pub async fn create_project(
    claims: Claims,
    State(project_service): State<Arc<dyn ProjectService>>,
    Json(req): Json<CreateProjectHttpRequest>,
) -> Result<Json<CreateProjectHttpResponse>, ApiError> {
    let project = project_service
        .create_project(&CreateProjectRequest {
            owner_id: claims.sub,
            name: req.project_name,
        })
        .await?;
    Ok(Json(CreateProjectHttpResponse {
        project_id: project.id,
    }))
}

#[axum::debug_handler(state = AppState)]
pub async fn get_project(
    _claims: Claims,
    State(project_service): State<Arc<dyn ProjectService>>,
    Path(project_id): Path<ProjectId>,
) -> Result<Json<GetProjectHttpResponse>, ApiError> {
    println!("->> HANDLER - get_project");
    let project = project_service.get_project_by_id(&project_id).await?;
    Ok(Json(project.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn list_projects(
    _claims: Claims,
    State(project_service): State<Arc<dyn ProjectService>>,
) -> Result<Json<ListProjectsHttpResponse>, ApiError> {
    let projects = project_service.list_projects().await?;
    Ok(Json(projects.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn delete_project(
    _claims: Claims,
    State(project_service): State<Arc<dyn ProjectService>>,
    Path(project_id): Path<ProjectId>,
) -> Result<(), ApiError> {
    project_service.delete_project(&project_id).await?;
    Ok(())
}
