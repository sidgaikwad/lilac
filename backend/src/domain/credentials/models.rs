use std::fmt::Display;

use crate::domain::serialize_secret_string;
use chrono::{DateTime, Utc};
use google_cloud_auth::credentials::{service_account::Builder, Credentials as GcpCredentials};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct CredentialId(pub Uuid);

impl CredentialId {
    pub fn new(id: Uuid) -> Self {
        Self(id)
    }

    pub fn generate() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn inner(&self) -> &Uuid {
        &self.0
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Display for CredentialId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Uuid> for CredentialId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl From<CredentialId> for Uuid {
    fn from(id: CredentialId) -> Self {
        id.0
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "credentials_type")]
pub enum Credentials {
    Aws {
        access_key: String,
        #[serde(serialize_with = "serialize_secret_string")]
        secret_key: SecretString,
    },
    Gcp(GoogleCredentials),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoogleCredentials {
    #[serde(rename = "type")]
    pub r#type: String,
    pub project_id: String,
    pub private_key_id: String,
    #[serde(serialize_with = "serialize_secret_string")]
    pub private_key: SecretString,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_x509_cert_url: String,
}

impl TryFrom<GoogleCredentials> for GcpCredentials {
    type Error = anyhow::Error;

    fn try_from(value: GoogleCredentials) -> Result<Self, Self::Error> {
        let json = serde_json::to_value(value)?;
        Ok(Builder::new(json).build()?)
    }
}

#[derive(Clone, Debug)]
pub struct Credential {
    pub id: CredentialId,
    pub name: String,
    pub description: Option<String>,
    pub credentials: Credentials,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct CreateCredentialRequest {
    pub name: String,
    pub description: Option<String>,
    pub credentials: Credentials,
}
