use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    domain::{
        auth::models::Claims,
        credentials::{models::CredentialId, service::CredentialService},
    },
    inbound::http::{
        errors::ApiError,
        routes::credentials::models::{
            CreateCredentialHttpResponse, CreateCredentialsHttpRequest, GetCredentialsHttpResponse,
            ListCredentialsHttpResponse,
        },
    },
};

use crate::inbound::http::AppState;

#[axum::debug_handler(state = AppState)]
pub async fn create_credential(
    _claims: Claims,
    State(credential_service): State<Arc<dyn CredentialService>>,
    Json(req): Json<CreateCredentialsHttpRequest>,
) -> Result<Json<CreateCredentialHttpResponse>, ApiError> {
    let credential = credential_service.create_credential(&req.into()).await?;
    Ok(Json(CreateCredentialHttpResponse {
        credential_id: credential.id,
    }))
}

#[axum::debug_handler(state = AppState)]
pub async fn get_credential(
    _claims: Claims,
    State(credential_service): State<Arc<dyn CredentialService>>,
    Path(credential_id): Path<CredentialId>,
) -> Result<Json<GetCredentialsHttpResponse>, ApiError> {
    let credential = credential_service
        .get_credential_by_id(&credential_id.into())
        .await?;
    Ok(Json(credential.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn list_credentials(
    _claims: Claims,
    State(credential_service): State<Arc<dyn CredentialService>>,
) -> Result<Json<ListCredentialsHttpResponse>, ApiError> {
    let credentials = credential_service.list_credentials().await?;
    Ok(Json(credentials.into()))
}

#[axum::debug_handler(state = AppState)]
pub async fn delete_credential(
    _claims: Claims,
    State(credential_service): State<Arc<dyn CredentialService>>,
    Path(credential_id): Path<Uuid>,
) -> Result<(), ApiError> {
    credential_service
        .delete_credential(&credential_id.into())
        .await?;
    Ok(())
}
