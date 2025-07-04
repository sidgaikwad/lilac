use std::sync::LazyLock;

use axum::{
    extract::{Path, State},
    Json,
};
use common::{
    database::Database,
    k8s::{helm::Helm, K8sWrapper},
    model::{
        organization::OrganizationId,
        service::{Service, ServiceId, ServiceType},
    },
    ServiceError,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use validator::Validate;

#[instrument(level = "info", skip(db, k8s), ret, err)]
pub async fn start_service(
    State(db): State<Database>,
    State(k8s): State<K8sWrapper>,
    Path(org_id): Path<OrganizationId>,
    Json(request): Json<StartServiceRequest>,
) -> Result<Json<StartServiceResponse>, ServiceError> {
    let org = db.get_organization(&org_id).await?;
    match request {
        StartServiceRequest::Airflow(request) => {
            request
                .validate()
                .map_err(|_| ServiceError::BadRequest("invalid request".into()))?;
            let service = Service::create(
                request.service_name.clone(),
                org.organization_id.clone(),
                ServiceType::Airflow {},
            );
            k8s.helm_install(
                &org.organization_id.to_string(),
                &request.service_name,
                "airflow",
            )
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                ServiceError::UnhandledError
            })?;
            let service_id = db.create_service(service).await?;
            Ok(Json(StartServiceResponse { service_id }))
        }
        StartServiceRequest::JupyterHub(request) => {
            request
                .validate()
                .map_err(|_| ServiceError::BadRequest("invalid request".into()))?;
            let service = Service::create(
                request.service_name.clone(),
                org.organization_id.clone(),
                ServiceType::JupyterHub {},
            );
            k8s.helm_install(
                &org.organization_id.to_string(),
                &request.service_name,
                "oci://registry-1.docker.io/bitnamicharts/jupyterhub",
            )
            .await
            .map_err(|e| {
                tracing::error!("{e}");
                ServiceError::UnhandledError
            })?;
            let service_id = db.create_service(service).await?;
            Ok(Json(StartServiceResponse { service_id }))
        }
        StartServiceRequest::Noop { service_name } => {
            let service = Service::create(
                service_name.clone(),
                org.organization_id.clone(),
                ServiceType::Unknown,
            );
            let service_id = db.create_service(service).await?;
            Ok(Json(StartServiceResponse { service_id }))
        }
    }
}

static NAME_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[a-zA-Z0-9_\-]+$").unwrap());

#[derive(Debug, Deserialize, Validate)]
pub struct AirflowRequest {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"), regex(path = *NAME_REGEX))]
    service_name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct JupyterHubRequest {
    #[validate(length(min = 3, message = "Name must be at least 3 characters"), regex(path = *NAME_REGEX))]
    service_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "service")]
pub enum StartServiceRequest {
    #[serde(rename = "airflow")]
    Airflow(AirflowRequest),
    #[serde(rename = "jupyterhub")]
    JupyterHub(JupyterHubRequest),
    #[serde(rename = "noop")]
    Noop { service_name: String },
}

#[derive(Debug, Serialize)]
pub struct StartServiceResponse {
    service_id: ServiceId,
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn get_service(
    State(db): State<Database>,
    Path(service_id): Path<ServiceId>,
) -> Result<Json<GetServiceResponse>, ServiceError> {
    let service = db.get_service(&service_id).await?;
    Ok(Json(GetServiceResponse::from(service)))
}

#[derive(Debug, Serialize)]
pub struct GetServiceResponse {
    service_id: ServiceId,
    pub service_name: String,
    pub organization_id: OrganizationId,
    pub service_type: String,
    pub service_configuration: ServiceType,
}

impl From<Service> for GetServiceResponse {
    fn from(service: Service) -> Self {
        GetServiceResponse {
            service_id: service.service_id,
            service_name: service.service_name,
            organization_id: service.organization_id,
            service_type: service.service_type.get_type().to_string(),
            service_configuration: service.service_type,
        }
    }
}

#[instrument(level = "info", skip(db), ret, err)]
pub async fn list_services(
    State(db): State<Database>,
    Path(organization_id): Path<OrganizationId>,
) -> Result<Json<ListServicesResponse>, ServiceError> {
    let services = db.list_services(&organization_id).await?;
    Ok(Json(ListServicesResponse::from(services)))
}

#[derive(Debug, Serialize)]
pub struct ServiceSummary {
    pub service_id: ServiceId,
    pub organization_id: OrganizationId,
    pub service_name: String,
    pub service_type: String,
}

#[derive(Debug, Serialize)]
pub struct ListServicesResponse {
    pub services: Vec<ServiceSummary>,
}

impl From<Vec<Service>> for ListServicesResponse {
    fn from(services: Vec<Service>) -> Self {
        ListServicesResponse {
            services: services
                .into_iter()
                .map(|v| ServiceSummary {
                    service_id: v.service_id,
                    organization_id: v.organization_id,
                    service_name: v.service_name,
                    service_type: v.service_type.get_type().to_string(),
                })
                .collect(),
        }
    }
}



#[instrument(level = "info", skip(db, k8s), ret, err)]
pub async fn delete_service(
    State(db): State<Database>,
    State(k8s): State<K8sWrapper>,
    Path(service_id): Path<ServiceId>,
) -> Result<(), ServiceError> {
    let service = db.get_service(&service_id).await?;

    k8s.helm_uninstall(&service.organization_id.to_string(), &service.service_name).await?;
    db.delete_service(&service_id).await?;
    Ok(())
}