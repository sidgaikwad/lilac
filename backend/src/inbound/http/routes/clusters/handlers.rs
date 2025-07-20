use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    domain::{
        auth::models::Claims,
        cluster::{models::ClusterId, service::ClusterService},
    },
    inbound::http::{
        errors::ApiError,
        routes::clusters::models::{
            CreateClusterHttpRequest, CreateClusterHttpResponse, GetClusterHttpResponse,
            ListClustersHttpResponse, TestClusterHttpRequest, TestClusterHttpResponse,
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

#[axum::debug_handler(state = AppState)]
pub async fn test_cluster_connection(
    State(cluster_service): State<Arc<dyn ClusterService>>,
    Json(req): Json<TestClusterHttpRequest>,
) -> Result<Json<TestClusterHttpResponse>, ApiError> {
    cluster_service
        .test_cluster_connection(req.credential_id, req.cluster_config.into())
        .await?;
    Ok(Json(TestClusterHttpResponse { success: true }))
}
