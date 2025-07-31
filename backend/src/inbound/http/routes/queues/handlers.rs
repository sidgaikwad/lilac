use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    domain::queue::models::{CreateQueueRequest, QueueId, UpdateQueueRequest},
    inbound::http::AppState,
};

use super::models::{HttpCreateQueueRequest, HttpQueueResponse, HttpUpdateQueueRequest};

pub async fn create_queue(
    State(state): State<AppState>,
    Json(request): Json<HttpCreateQueueRequest>,
) -> Result<Json<HttpQueueResponse>, StatusCode> {
    let new_queue = CreateQueueRequest {
        name: request.name,
        priority: request.priority,
        cluster_targets: request.cluster_targets,
    };

    match state.queue_service.create_queue(new_queue).await {
        Ok(queue) => Ok(Json(queue.into())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn list_queues(
    State(state): State<AppState>,
) -> Result<Json<Vec<HttpQueueResponse>>, StatusCode> {
    match state.queue_service.list_all_queues().await {
        Ok(queues) => {
            let response = queues.into_iter().map(|q| q.into()).collect();
            Ok(Json(response))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_queue(
    State(state): State<AppState>,
    Path(queue_id): Path<QueueId>,
) -> Result<Json<HttpQueueResponse>, StatusCode> {
    match state.queue_service.get_queue_by_id(&queue_id).await {
        Ok(Some(queue)) => Ok(Json(queue.into())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_queue(
    State(state): State<AppState>,
    Path(queue_id): Path<QueueId>,
    Json(request): Json<HttpUpdateQueueRequest>,
) -> Result<Json<HttpQueueResponse>, StatusCode> {
    let updated_queue = UpdateQueueRequest {
        id: queue_id,
        name: request.name,
        priority: request.priority,
        cluster_targets: request.cluster_targets,
    };

    match state.queue_service.update_queue(updated_queue).await {
        Ok(queue) => Ok(Json(queue.into())),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn delete_queue(
    State(state): State<AppState>,
    Path(queue_id): Path<QueueId>,
) -> Result<StatusCode, StatusCode> {
    match state.queue_service.delete_queue(&queue_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
