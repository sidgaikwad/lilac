use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    domain::{
        project::{
            models::{CreateProjectRequest, ProjectId},
            service::ProjectService,
        },
        user::models::UserId,
    },
    inbound::http::{responses::ApiError, AppState},
    domain::auth::models::Claims,
};

use axum::{
    routing::{get, post},
    Router,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/projects", post(create_project).get(list_projects))
        .route(
            "/projects/{id}",
            get(get_project).delete(delete_project),
        )
}

#[axum::debug_handler(state = AppState)]
pub async fn create_project(
    State(project_service): State<Arc<dyn ProjectService>>,
    claims: Claims,
    Json(req): Json<CreateProjectRequest>,
) -> Result<Json<crate::inbound::http::responses::ProjectResponse>, ApiError> {
    let project = project_service.create_project(&UserId(claims.sub), &req).await?;
    Ok(Json(project.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn get_project(
    State(project_service): State<Arc<dyn ProjectService>>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<Json<crate::inbound::http::responses::ProjectResponse>, ApiError> {
    let project = project_service
        .get_project_by_id(&UserId(claims.sub), &ProjectId(project_id))
        .await?;
    Ok(Json(project.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn list_projects(
    State(project_service): State<Arc<dyn ProjectService>>,
    claims: Claims,
) -> Result<Json<crate::inbound::http::responses::ListProjectsResponse>, ApiError> {
    let projects = project_service
        .list_projects_by_user_id(&UserId(claims.sub))
        .await?
        .into_iter()
        .map(|p| p.into())
        .collect();
    Ok(Json(crate::inbound::http::responses::ListProjectsResponse { projects }))
}

#[axum::debug_handler(state = AppState)]
pub async fn delete_project(
    State(project_service): State<Arc<dyn ProjectService>>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> Result<(), ApiError> {
    project_service
        .delete_project(&UserId(claims.sub), &ProjectId(project_id))
        .await?;
    Ok(())
}