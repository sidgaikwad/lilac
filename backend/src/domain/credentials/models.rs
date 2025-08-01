
use crate::{domain::serialize_secret_string, identifier};
use chrono::{DateTime, Utc};
use google_cloud_auth::credentials::{service_account::Builder, Credentials as GcpCredentials};
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

identifier!(CredentialId);

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
