use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::{
    domain::queue::service::{CreateQueue, UpdateQueue},
    inbound::http::AppState,
};

use super::models::{CreateQueueRequest, QueueResponse, UpdateQueueRequest};

pub async fn create_queue(
    State(state): State<AppState>,
    Json(request): Json<CreateQueueRequest>,
) -> Result<Json<QueueResponse>, StatusCode> {
    let new_queue = CreateQueue {
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
) -> Result<Json<Vec<QueueResponse>>, StatusCode> {
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
    Path(id): Path<Uuid>,
) -> Result<Json<QueueResponse>, StatusCode> {
    match state.queue_service.get_queue_by_id(id).await {
        Ok(Some(queue)) => Ok(Json(queue.into())),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn update_queue(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateQueueRequest>,
) -> Result<Json<QueueResponse>, StatusCode> {
    let updated_queue = UpdateQueue {
        id,
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
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    match state.queue_service.delete_queue(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}