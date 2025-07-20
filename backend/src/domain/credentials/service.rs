use std::sync::Arc;

use async_trait::async_trait;

use super::{
    models::{CreateCredentialRequest, Credential, CredentialId},
    ports::{CredentialRepository, CredentialRepositoryError},
};

#[derive(Debug, thiserror::Error)]
pub enum CredentialServiceError {
    #[error("invalid permissions")]
    InvalidPermissions,
    #[error("credential with {field} {value} already exists")]
    CredentialExists { field: String, value: String },
    #[error("credential {0} not found")]
    CredentialNotFound(String),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<CredentialRepositoryError> for CredentialServiceError {
    fn from(error: CredentialRepositoryError) -> Self {
        match error {
            CredentialRepositoryError::Duplicate { field, value } => {
                Self::CredentialExists { field, value }
            }
            CredentialRepositoryError::NotFound(id) => Self::CredentialNotFound(id),
            CredentialRepositoryError::Unknown(error) => Self::Unknown(error),
        }
    }
}

#[async_trait]
pub trait CredentialService: Send + Sync {
    async fn create_credential(
        &self,
        req: &CreateCredentialRequest,
    ) -> Result<Credential, CredentialServiceError>;
    async fn get_credential_by_id(
        &self,
        id: &CredentialId,
    ) -> Result<Credential, CredentialServiceError>;
    async fn list_credentials(&self) -> Result<Vec<Credential>, CredentialServiceError>;
    async fn delete_credential(&self, id: &CredentialId) -> Result<(), CredentialServiceError>;
}

#[derive(Clone)]
pub struct CredentialServiceImpl<R: CredentialRepository> {
    repo: Arc<R>,
}

impl<R: CredentialRepository> CredentialServiceImpl<R> {
    pub fn new(repo: Arc<R>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl<R: CredentialRepository> CredentialService for CredentialServiceImpl<R> {
    async fn create_credential(
        &self,
        req: &CreateCredentialRequest,
    ) -> Result<Credential, CredentialServiceError> {
        Ok(self.repo.create_credential(req).await?)
    }

    async fn get_credential_by_id(
        &self,
        id: &CredentialId,
    ) -> Result<Credential, CredentialServiceError> {
        Ok(self.repo.get_credential_by_id(id).await?)
    }

    async fn list_credentials(&self) -> Result<Vec<Credential>, CredentialServiceError> {
        Ok(self.repo.list_credentials().await?)
    }

    async fn delete_credential(&self, id: &CredentialId) -> Result<(), CredentialServiceError> {
        Ok(self.repo.delete_credential(id).await?)
    }
}
