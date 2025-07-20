use async_trait::async_trait;
use thiserror::Error;

use super::models::{CreateCredentialRequest, Credential, CredentialId};

#[derive(Debug, Error)]
pub enum CredentialRepositoryError {
    #[error("credential with {field} {value} already exists")]
    Duplicate { field: String, value: String },
    #[error("credential with id {0} not found")]
    NotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[async_trait]
pub trait CredentialRepository: Send + Sync {
    async fn create_credential(
        &self,
        req: &CreateCredentialRequest,
    ) -> Result<Credential, CredentialRepositoryError>;
    async fn get_credential_by_id(
        &self,
        id: &CredentialId,
    ) -> Result<Credential, CredentialRepositoryError>;
    async fn list_credentials(&self) -> Result<Vec<Credential>, CredentialRepositoryError>;
    async fn delete_credential(&self, id: &CredentialId) -> Result<(), CredentialRepositoryError>;
}

#[derive(Debug, Error)]
pub enum CredentialConnectionError {
    #[error("invalid credentials: {0}")]
    InvalidCredentials(String),
    #[error("could not reach credential: {0}")]
    CredentialUnreachable(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}
