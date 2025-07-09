use std::sync::LazyLock;

use axum::{
    extract::{Path, State},
    Json,
};
use common::{
    database::Database,
    k8s::{
        gateway_api::{
            HTTPBackendRef, HTTPPathMatch, HTTPRoute, HTTPRouteFilter, HTTPRouteMatch,
            HTTPRouteRule, HTTPRouteSpec, ParentReference, PathRewrite, URLRewrite,
        },
        helm::Helm,
        K8sApi, K8sWrapper,
    },
    model::{
        project::ProjectId,
        service::{Service, ServiceId, ServiceType},
    },
    ServiceError,
};
use kube::api::ObjectMeta;
use regex::Regex;
use serde::{Deserialize, Serialize};
use tracing::instrument;
use validator::Validate;

use crate::auth::claims::Claims;
use crate::AppState;
use crate::ServiceConfig;

#[instrument(level = "info", skip(app_state), ret, err)]
pub async fn start_service(
    claims: Claims,
    State(app_state): State<AppState>,
    Path(project_id): Path<ProjectId>,
    Json(request): Json<StartServiceRequest>,
) -> Result<Json<StartServiceResponse>, ServiceError> {
    let db = &app_state.db;
    let k8s = &app_state.k8s;
    let service_config = &app_state.service_config;

    let is_member = db.is_user_project_member(&claims.sub, &project_id).await?;

    if !is_member {
        return Err(ServiceError::Unauthorized);
    }

    let project = db.get_project(&project_id).await?;

    let (service_name, service_type, chart, port) = match request {
        StartServiceRequest::Airflow(req) => {
            req.validate()
                .map_err(|_| ServiceError::BadRequest("invalid request".into()))?;
            (
                req.service_name,
                ServiceType::Airflow {},
                "airflow",
                8080,
            )
        }
        StartServiceRequest::JupyterHub(req) => {
            req.validate()
                .map_err(|_| ServiceError::BadRequest("invalid request".into()))?;
            (
                req.service_name,
                ServiceType::JupyterHub {},
                "jupyterhub",
                8000,
            )
        }
        StartServiceRequest::MLflow(req) => {
            req.validate()
                .map_err(|_| ServiceError::BadRequest("invalid request".into()))?;
            (
                req.service_name,
                ServiceType::MLflow {},
                "community-charts/mlflow",
                5000,
            )
        }
        StartServiceRequest::Noop { service_name } => {
            let service =
                Service::create(service_name, project.project_id.clone(), ServiceType::Unknown);
            let service_id = db.create_service(service).await?;
            return Ok(Json(StartServiceResponse { service_id }));
        }
    };

    let service_id = provision_service(
        &db,
        &k8s,
        &service_config,
        project.project_id,
        service_name,
        service_type,
        chart,
        port,
    )
    .await?;

    Ok(Json(StartServiceResponse { service_id }))
}

#[allow(clippy::too_many_arguments)]
async fn provision_service(
    db: &Database,
    k8s: &K8sWrapper,
    service_config: &ServiceConfig,
    project_id: ProjectId,
    service_name: String,
    service_type: ServiceType,
    chart: &str,
    port: u16,
) -> Result<ServiceId, ServiceError> {
    let project_id_str = project_id.to_string();

    let values_owned = if let ServiceType::MLflow {} = service_type {
        vec![
            format!("backendStore.postgresql.postgresqlUsername={}", "lilac-user"),
            format!("backendStore.postgresql.postgresqlPassword={}", "password123"),
            format!(
                "backendStore.postgresql.postgresqlDatabase={}",
                "lilac"
            ),
            format!(
                "backendStore.postgresql.postgresqlHost={}",
                "lilac-postgresql.lilac"
            ),
        ]
    } else {
        vec![]
    };

    let values: Option<Vec<&str>> = if !values_owned.is_empty() {
        Some(values_owned.iter().map(|s| s.as_str()).collect())
    } else {
        None
    };

    k8s.helm_install(&project_id_str, &service_name, chart, values)
        .await
        .map_err(|e| {
            tracing::error!("Helm install failed: {}", e);
            ServiceError::UnhandledError
        })?;

    let route = HTTPRoute {
        metadata: ObjectMeta {
            name: Some(service_name.clone()),
            namespace: Some(project_id_str.clone()),
            ..Default::default()
        },
        spec: HTTPRouteSpec {
            parent_refs: Some(vec![ParentReference {
                group: Some("gateway.networking.k8s.io".to_string()),
                kind: Some("Gateway".to_string()),
                name: "lilac-gateway".to_string(),
                namespace: Some("lilac".to_string()),
            }]),
            rules: Some(vec![HTTPRouteRule {
                matches: Some(vec![HTTPRouteMatch {
                    path: Some(HTTPPathMatch {
                        value: Some(format!("/projects/{}/services/{}", project_id_str, service_name)),
                    }),
                }]),
                backend_refs: Some(vec![HTTPBackendRef {
                    name: format!("{}-{}", service_name, service_type.get_type()),
                    port,
                }]),
                filters: Some(vec![HTTPRouteFilter {
                    r#type: "URLRewrite".to_string(),
                    url_rewrite: Some(URLRewrite {
                        path: Some(PathRewrite {
                            r#type: "ReplacePrefixMatch".to_string(),
                            replace_prefix_match: "/".to_string(),
                        }),
                    }),
                }]),
            }]),
            hostnames: None,
        },
    };

    if let Err(e) = k8s.create_http_route(&project_id_str, &route).await {
        tracing::error!("Failed to create http route: {}", e);
        k8s.helm_uninstall(&project_id_str, &service_name).await?;
        return Err(e.into());
    }

    let public_url = format!(
        "{}/projects/{}/services/{}",
        service_config.gateway_url, project_id_str, service_name
    );
    let mut service = Service::create(service_name.clone(), project_id, service_type);
    service.url = Some(public_url);

    match db.create_service(service).await {
        Ok(service_id) => Ok(service_id),
        Err(e) => {
            tracing::error!("Failed to create service in db: {}", e);
            k8s.helm_uninstall(&project_id_str, &service_name).await?;
            Err(e)
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

#[derive(Debug, Deserialize, Validate)]
pub struct MLflowRequest {
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
    #[serde(rename = "mlflow")]
    MLflow(MLflowRequest),
    #[serde(rename = "noop")]
    Noop { service_name: String },
}

#[derive(Debug, Serialize)]
pub struct StartServiceResponse {
    service_id: ServiceId,
}

#[instrument(level = "info", skip(app_state), ret, err)]
pub async fn get_service(
    claims: Claims,
    State(app_state): State<AppState>,
    Path(service_id): Path<ServiceId>,
) -> Result<Json<GetServiceResponse>, ServiceError> {
    let db = &app_state.db;
    let service = db.get_service(&service_id).await?;

    let is_member = db
        .is_user_project_member(&claims.sub, &service.project_id)
        .await?;

    if !is_member {
        return Err(ServiceError::Unauthorized);
    }

    Ok(Json(GetServiceResponse::from(service)))
}

#[derive(Debug, Serialize)]
pub struct GetServiceResponse {
    service_id: ServiceId,
    pub service_name: String,
    pub project_id: ProjectId,
    pub service_type: String,
    pub service_configuration: ServiceType,
    pub url: Option<String>,
}

impl From<Service> for GetServiceResponse {
    fn from(service: Service) -> Self {
        GetServiceResponse {
            service_id: service.service_id,
            service_name: service.service_name,
            project_id: service.project_id,
            service_type: service.service_type.get_type().to_string(),
            service_configuration: service.service_type,
            url: service.url,
        }
    }
}

#[instrument(level = "info", skip(app_state), ret, err)]
pub async fn list_services(
    claims: Claims,
    State(app_state): State<AppState>,
    Path(project_id): Path<ProjectId>,
) -> Result<Json<ListServicesResponse>, ServiceError> {
    let db = &app_state.db;
    let is_member = db.is_user_project_member(&claims.sub, &project_id).await?;

    if !is_member {
        return Err(ServiceError::Unauthorized);
    }

    let services = db.list_services(&project_id).await?;
    Ok(Json(ListServicesResponse::from(services)))
}

#[derive(Debug, Serialize)]
pub struct ServiceSummary {
    pub service_id: ServiceId,
    pub project_id: ProjectId,
    pub service_name: String,
    pub service_type: String,
    pub url: Option<String>,
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
                    project_id: v.project_id,
                    service_name: v.service_name,
                    service_type: v.service_type.get_type().to_string(),
                    url: v.url,
                })
                .collect(),
        }
    }
}

#[instrument(level = "info", skip(app_state), ret, err)]
pub async fn delete_service(
    claims: Claims,
    State(app_state): State<AppState>,
    Path(service_id): Path<ServiceId>,
) -> Result<(), ServiceError> {
    let db = &app_state.db;
    let k8s = &app_state.k8s;
    let service = db.get_service(&service_id).await?;

    let is_member = db
        .is_user_project_member(&claims.sub, &service.project_id)
        .await?;

    if !is_member {
        return Err(ServiceError::Unauthorized);
    }

    k8s.helm_uninstall(&service.project_id.to_string(), &service.service_name)
        .await?;
    db.delete_service(&service_id).await?;
    Ok(())
}
