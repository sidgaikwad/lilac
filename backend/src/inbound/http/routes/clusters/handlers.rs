use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use secrecy::{ExposeSecret, SecretString};
use uuid::Uuid;

use crate::{
    domain::{
        auth::models::Claims,
        cluster::{
            models::{ClusterId, NodeId, UpdateNodeStatusRequest},
            service::ClusterService,
        },
        user::models::NewApiKey,
    },
    inbound::http::{
        errors::ApiError,
        routes::clusters::models::{
            CreateClusterHttpRequest, CreateClusterHttpResponse, GetClusterHttpResponse, HttpApiKey,
            HttpClusterNodeDetails, HttpClusterNodeHeartbeat, ListClusterNodesHttpResponse,
            ListClustersHttpResponse,
        },
    },
};

use crate::inbound::http::AppState;

#[axum::debug_handler(state = AppState)]
pub async fn create_cluster(
    _claims: Claims,
    State(cluster_service): State<Arc<dyn ClusterService>>,
    Json(req): Json<CreateClusterHttpRequest>,
) -> Result<Json<CreateClusterHttpResponse>, ApiError> {
    let cluster = cluster_service.create_cluster(&req.into()).await?;
    Ok(Json(CreateClusterHttpResponse {
        cluster_id: cluster.id,
    }))
}

#[axum::debug_handler(state = AppState)]
pub async fn get_cluster(
    _claims: Claims,
    State(cluster_service): State<Arc<dyn ClusterService>>,
    Path(cluster_id): Path<ClusterId>,
) -> Result<Json<GetClusterHttpResponse>, ApiError> {
    let cluster = cluster_service
        .get_cluster_by_id(&cluster_id.into())
        .await?;
    Ok(Json(cluster.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn list_clusters(
    _claims: Claims,
    State(cluster_service): State<Arc<dyn ClusterService>>,
) -> Result<Json<ListClustersHttpResponse>, ApiError> {
    let clusters = cluster_service.list_clusters().await?;
    Ok(Json(clusters.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn delete_cluster(
    _claims: Claims,
    State(cluster_service): State<Arc<dyn ClusterService>>,
    Path(cluster_id): Path<Uuid>,
) -> Result<(), ApiError> {
    cluster_service.delete_cluster(&cluster_id.into()).await?;
    Ok(())
}

use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};

#[axum::debug_handler(state = AppState)]
pub async fn create_api_key_for_cluster(
    _claims: Claims,
    State(cluster_service): State<Arc<dyn ClusterService>>,
    Path(cluster_id): Path<ClusterId>,
) -> Result<Json<NewApiKey>, ApiError> {
    let new_api_key = cluster_service
        .create_api_key_for_cluster(&cluster_id)
        .await?;
    Ok(Json(new_api_key))
}

#[axum::debug_handler(state = AppState)]
pub async fn list_api_keys(
    _claims: Claims,
    State(cluster_service): State<Arc<dyn ClusterService>>,
    Path(cluster_id): Path<ClusterId>,
) -> Result<Json<Vec<HttpApiKey>>, ApiError> {
    let api_keys = cluster_service.list_api_keys(&cluster_id).await?;
    let http_api_keys = api_keys.into_iter().map(HttpApiKey::from).collect();
    Ok(Json(http_api_keys))
}

#[axum::debug_handler(state = AppState)]
pub async fn cluster_node_heartbeat(
    Path(node_id): Path<NodeId>,
    State(cluster_service): State<Arc<dyn ClusterService>>,
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(req): Json<HttpClusterNodeHeartbeat>,
) -> Result<Json<HttpClusterNodeDetails>, ApiError> {
    let cluster = cluster_service
        .authenticate_by_api_key(&SecretString::from(auth.token().to_string()))
        .await?;

    let _resp = cluster_service
        .update_node_status(UpdateNodeStatusRequest {
            node_id,
            cluster_id: cluster.id,
            status: req.status,
            heartbeat_timestamp: Utc::now(),
            memory_info: req.memory_info,
            cpu_info: req.cpu_info,
            gpu_info: req.gpu_info,
            job_info: req.job_info,
        })
        .await?;
    Ok(Json(HttpClusterNodeDetails {})) // TODO: reply with job assignment info
}

#[axum::debug_handler(state = AppState)]
pub async fn list_cluster_nodes(
    _claims: Claims,
    State(cluster_service): State<Arc<dyn ClusterService>>,
    Path(cluster_id): Path<ClusterId>,
) -> Result<Json<ListClusterNodesHttpResponse>, ApiError> {
    let cluster_nodes = cluster_service
        .list_cluster_nodes(&cluster_id.into())
        .await?;
    Ok(Json(cluster_nodes.into()))
}
