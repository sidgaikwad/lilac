use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{
    auth::models::Claims,
    service::{
        models::{CreateService, UpdateService},
        ports::ServiceService,
    },
    user::models::UserId,
};

use super::super::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/projects/{project_id}/services",
            get(list_services).post(create_service),
        )
        .route(
            "/projects/{project_id}/services/{service_id}",
            get(get_service)
                .patch(update_service)
                .delete(delete_service),
        )
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateServiceRequest {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateServiceRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}

#[axum::debug_handler(state = AppState)]
pub async fn create_service(
    State(service): State<Arc<dyn ServiceService>>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
    Json(request): Json<CreateServiceRequest>,
) -> impl IntoResponse {
    let create_service = CreateService {
        project_id,
        name: request.name,
        description: request.description,
        url: request.url,
    };

    match service
        .create_service(&UserId(claims.sub), &create_service)
        .await
    {
        Ok(created) => (StatusCode::CREATED, Json(created)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[axum::debug_handler(state = AppState)]
pub async fn list_services(
    State(service): State<Arc<dyn ServiceService>>,
    claims: Claims,
    Path(project_id): Path<Uuid>,
) -> impl IntoResponse {
    match service
        .get_services_by_project_id(&UserId(claims.sub), project_id)
        .await
    {
        Ok(services) => (StatusCode::OK, Json(services)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[axum::debug_handler(state = AppState)]
pub async fn get_service(
    State(service): State<Arc<dyn ServiceService>>,
    claims: Claims,
    Path((_project_id, service_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    match service
        .get_service_by_id(&UserId(claims.sub), service_id)
        .await
    {
        Ok(Some(found)) => (StatusCode::OK, Json(found)).into_response(),
        Ok(None) => (StatusCode::NOT_FOUND).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[axum::debug_handler(state = AppState)]
pub async fn update_service(
    State(service): State<Arc<dyn ServiceService>>,
    claims: Claims,
    Path((_project_id, service_id)): Path<(Uuid, Uuid)>,
    Json(request): Json<UpdateServiceRequest>,
) -> impl IntoResponse {
    let update_service = UpdateService {
        name: request.name,
        description: request.description,
        url: request.url,
    };

    match service
        .update_service(&UserId(claims.sub), service_id, &update_service)
        .await
    {
        Ok(updated) => (StatusCode::OK, Json(updated)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

#[axum::debug_handler(state = AppState)]
pub async fn delete_service(
    State(service): State<Arc<dyn ServiceService>>,
    claims: Claims,
    Path((_project_id, service_id)): Path<(Uuid, Uuid)>,
) -> impl IntoResponse {
    match service
        .delete_service(&UserId(claims.sub), service_id)
        .await
    {
        Ok(_) => (StatusCode::NO_CONTENT).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
