use serde::{Deserialize, Serialize};

use crate::domain::{
    project::models::{Project, ProjectId},
    user::models::UserId,
};

/// The body of a [Project] creation request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateProjectHttpRequest {
    pub project_name: String,
}

/// The body of a [Project] creation response.
#[derive(Debug, Clone, Serialize)]
pub struct CreateProjectHttpResponse {
    pub project_id: ProjectId,
}

/// The body of a [Project] get response.
#[derive(Debug, Clone, Serialize)]
pub struct GetProjectHttpResponse {
    pub project_id: ProjectId,
    pub owner_id: UserId,
    pub project_name: String,
}

impl From<Project> for GetProjectHttpResponse {
    fn from(project: Project) -> Self {
        Self {
            project_id: project.id,
            owner_id: project.owner_id,
            project_name: project.name,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct HttpProjectSummary {
    pub project_id: ProjectId,
    pub owner_id: UserId,
    pub project_name: String,
}

impl From<Project> for HttpProjectSummary {
    fn from(project: Project) -> Self {
        Self {
            project_id: project.id,
            owner_id: project.owner_id,
            project_name: project.name,
        }
    }
}

/// The body of a [Project] list response.
#[derive(Clone, Debug, Serialize)]
pub struct ListProjectsHttpResponse {
    pub projects: Vec<HttpProjectSummary>,
}

impl From<Vec<Project>> for ListProjectsHttpResponse {
    fn from(projects: Vec<Project>) -> Self {
        Self {
            projects: projects.into_iter().map(HttpProjectSummary::from).collect(),
        }
    }
}
