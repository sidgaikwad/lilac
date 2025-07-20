use secrecy::SecretString;
use serde::{Deserialize, Serialize};

use crate::domain::credentials::models::{
    CreateCredentialRequest, Credential, CredentialId, Credentials,
};
use crate::domain::serialize_secret_string;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "credential_type")]
pub enum HttpCredentials {
    #[serde(rename = "aws")]
    Aws {
        access_key: String,
        #[serde(serialize_with = "serialize_secret_string")]
        secret_key: SecretString,
    },
}

impl From<HttpCredentials> for Credentials {
    fn from(value: HttpCredentials) -> Self {
        match value {
            HttpCredentials::Aws {
                access_key,
                secret_key,
            } => Self::Aws {
                access_key,
                secret_key,
            },
        }
    }
}

impl From<Credentials> for HttpCredentials {
    fn from(value: Credentials) -> Self {
        match value {
            Credentials::Aws {
                access_key,
                secret_key,
            } => Self::Aws {
                access_key,
                secret_key,
            },
        }
    }
}

/// The body of a [Credential] creation request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCredentialsHttpRequest {
    credential_name: String,
    credential_description: Option<String>,
    credentials: HttpCredentials,
}

impl From<CreateCredentialsHttpRequest> for CreateCredentialRequest {
    fn from(value: CreateCredentialsHttpRequest) -> Self {
        CreateCredentialRequest {
            name: value.credential_name,
            description: value.credential_description,
            credentials: value.credentials.into(),
        }
    }
}

/// The body of a [Credential] creation response.
#[derive(Debug, Clone, Serialize)]
pub struct CreateCredentialHttpResponse {
    pub credential_id: CredentialId,
}

/// The body of a [Credential] get response.
#[derive(Debug, Clone, Serialize)]
pub struct GetCredentialsHttpResponse {
    pub credential_id: CredentialId,
    pub credential_name: String,
    pub credential_description: Option<String>,
    pub credentials: HttpCredentials,
}

impl From<Credential> for GetCredentialsHttpResponse {
    fn from(value: Credential) -> Self {
        Self {
            credential_id: value.id,
            credential_name: value.name,
            credential_description: value.description,
            credentials: value.credentials.into(),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct HttpCredentialsSummary {
    pub credential_id: CredentialId,
    pub credential_name: String,
    pub credential_description: Option<String>,
    pub credential_type: String,
}

impl From<Credential> for HttpCredentialsSummary {
    fn from(credential: Credential) -> Self {
        let credential_type = match credential.credentials {
            Credentials::Aws { .. } => "aws".to_string(),
        };

        Self {
            credential_id: credential.id,
            credential_name: credential.name,
            credential_description: credential.description,
            credential_type,
        }
    }
}

/// The body of a [Credential] list response.
#[derive(Clone, Debug, Serialize)]
pub struct ListCredentialsHttpResponse {
    pub credentials: Vec<HttpCredentialsSummary>,
}

impl From<Vec<Credential>> for ListCredentialsHttpResponse {
    fn from(value: Vec<Credential>) -> Self {
        Self {
            credentials: value
                .into_iter()
                .map(HttpCredentialsSummary::from)
                .collect(),
        }
    }
}
