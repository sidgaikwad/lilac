use std::fmt::Display;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::ServiceError;

use super::project::ProjectId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type)]
#[sqlx(transparent)]
pub struct ServiceId(Uuid);

impl ServiceId {
    pub fn new(service_id: Uuid) -> Self {
        Self(service_id)
    }

    pub fn generate() -> Self {
        let id = Uuid::new_v4();
        Self(id)
    }

    pub fn inner(&self) -> &Uuid {
        &self.0
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Default for ServiceId {
    fn default() -> Self {
        Self::generate()
    }
}

impl Display for ServiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for ServiceId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl TryFrom<String> for ServiceId {
    type Error = ServiceError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let id =
            Uuid::try_parse(&value).map_err(|_| ServiceError::ParseError("ServiceId".into()))?;
        Ok(Self(id))
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(tag = "service_type", rename_all = "snake_case")]
pub enum ServiceType {
    Airflow {},
    JupyterHub {},
    MLflow {},
    #[default]
    Unknown,
}

impl ServiceType {
    pub fn get_type(&self) -> &'static str {
        match self {
            ServiceType::Airflow {} => "airflow",
            Self::JupyterHub {} => "jupyterhub",
            Self::MLflow {} => "mlflow",
            ServiceType::Unknown => "unknown",
        }
    }
}

#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct Service {
    pub service_id: ServiceId,
    pub service_name: String,
    pub project_id: ProjectId,
    pub service_type: ServiceType,
    pub url: Option<String>,
}

impl Service {
    pub fn new(
        service_id: ServiceId,
        service_name: String,
        project_id: ProjectId,
        service_type: ServiceType,
    ) -> Self {
        Self {
            service_id,
            service_name,
            project_id,
            service_type,
            url: None,
        }
    }

    pub fn create(
        service_name: String,
        project_id: ProjectId,
        service_type: ServiceType,
    ) -> Self {
        Self {
            service_id: ServiceId::generate(),
            service_name,
            project_id,
            service_type,
            url: None,
        }
    }
}
