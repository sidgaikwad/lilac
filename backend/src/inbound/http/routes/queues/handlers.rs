use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    domain::queue::{
        models::{CreateQueueRequest, QueueId, UpdateQueueRequest},
        service::QueueService,
    },
    inbound::http::{
        errors::ApiError, routes::training_jobs::models::ListTrainingJobsHttpResponse,
    },
};

use super::models::{HttpCreateQueueRequest, HttpQueueResponse, HttpUpdateQueueRequest};

pub async fn create_queue(
    State(queue_service): State<Arc<dyn QueueService>>,
    Json(request): Json<HttpCreateQueueRequest>,
) -> Result<Json<HttpQueueResponse>, ApiError> {
    let new_queue = CreateQueueRequest {
        name: request.name,
        priority: request.priority,
        cluster_targets: request.cluster_targets,
    };

    let queue = queue_service.create_queue(new_queue).await?;
    Ok(Json(queue.into()))
}

pub async fn list_queues(
    State(queue_service): State<Arc<dyn QueueService>>,
) -> Result<Json<Vec<HttpQueueResponse>>, ApiError> {
    let queues = queue_service.list_all_queues().await?;
    let response = queues.into_iter().map(|q| q.into()).collect();
    Ok(Json(response))
}

pub async fn list_queue_jobs(
    State(queue_service): State<Arc<dyn QueueService>>,
    Path(queue_id): Path<QueueId>,
) -> Result<Json<ListTrainingJobsHttpResponse>, ApiError> {
    let jobs = queue_service.list_queues_jobs(&queue_id).await?;
    Ok(Json(jobs.into()))
}

pub async fn get_queue(
    State(queue_service): State<Arc<dyn QueueService>>,
    Path(queue_id): Path<QueueId>,
) -> Result<Json<HttpQueueResponse>, ApiError> {
    let queue = queue_service.get_queue_by_id(&queue_id).await?;
    Ok(Json(queue.into()))
}

pub async fn update_queue(
    State(queue_service): State<Arc<dyn QueueService>>,
    Path(queue_id): Path<QueueId>,
    Json(request): Json<HttpUpdateQueueRequest>,
) -> Result<Json<HttpQueueResponse>, ApiError> {
    let updated_queue = UpdateQueueRequest {
        id: queue_id,
        name: request.name,
        priority: request.priority,
        cluster_targets: request.cluster_targets,
    };

    let queue = queue_service.update_queue(updated_queue).await?;
    Ok(Json(queue.into()))
}

pub async fn delete_queue(
    State(queue_service): State<Arc<dyn QueueService>>,
    Path(queue_id): Path<QueueId>,
) -> Result<(), ApiError> {
    queue_service.delete_queue(&queue_id).await?;
    Ok(())
}
