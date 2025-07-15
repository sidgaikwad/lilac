use std::sync::Arc;

use async_trait::async_trait;

use super::{
    models::{
        AWSIntegration, CreateAWSIntegrationRequest, Integration, IntegrationId,
    },
    ports::{IntegrationRepository, IntegrationRepositoryError, StsPort},
};
use crate::domain::{
    project::models::ProjectId,
    user::models::UserId,
};

#[async_trait]
pub trait IntegrationService: Send + Sync {
    async fn create_aws_integration(
        &self,
        user_id: &UserId,
        req: &CreateAWSIntegrationRequest,
    ) -> Result<AWSIntegration, IntegrationRepositoryError>;

    async fn get_integrations_by_project_id(
        &self,
        user_id: &UserId,
        project_id: &ProjectId,
    ) -> Result<Vec<Integration>, IntegrationRepositoryError>;

    async fn get_integration_by_id(
        &self,
        user_id: &UserId,
        integration_id: &IntegrationId,
    ) -> Result<Integration, IntegrationRepositoryError>;
}

#[derive(Clone)]
pub struct IntegrationServiceImpl<
    R: IntegrationRepository,
    S: StsPort,
> {
    repo: Arc<R>,
    sts: Arc<S>,
}

impl<R: IntegrationRepository, S: StsPort>
    IntegrationServiceImpl<R, S>
{
    pub fn new(repo: Arc<R>, sts: Arc<S>) -> Self {
        Self {
            repo,
            sts,
        }
    }
}

#[async_trait]
impl<R: IntegrationRepository, S: StsPort> IntegrationService
    for IntegrationServiceImpl<R, S>
{
    async fn create_aws_integration(
        &self,
        user_id: &UserId,
        req: &CreateAWSIntegrationRequest,
    ) -> Result<AWSIntegration, IntegrationRepositoryError> {
        self.sts
            .assume_role(&req.role_arn, "lilac-placeholder")
            .await
            .map_err(|e| IntegrationRepositoryError::Unknown(e.into()))?;

        self.repo.create_aws_integration(user_id, req).await
    }

    async fn get_integrations_by_project_id(
        &self,
        user_id: &UserId,
        project_id: &ProjectId,
    ) -> Result<Vec<Integration>, IntegrationRepositoryError> {
        self.repo
            .get_integrations_by_project_id(user_id, project_id)
            .await
    }

    async fn get_integration_by_id(
        &self,
        user_id: &UserId,
        integration_id: &IntegrationId,
    ) -> Result<Integration, IntegrationRepositoryError> {
        self.repo
            .get_integration_by_id(user_id, integration_id)
            .await
    }
}