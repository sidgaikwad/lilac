use serde::{Deserialize, Serialize};

use crate::domain::{
    cluster::models::{Cluster, ClusterConfig, ClusterId, CreateClusterRequest},
    credentials::models::CredentialId,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "cluster_type")]
pub enum HttpClusterConfig {
    #[serde(rename = "aws_eks")]
    AwsEks {
        cluster_name: String,
        region: String,
    },
}

impl From<HttpClusterConfig> for ClusterConfig {
    fn from(value: HttpClusterConfig) -> Self {
        match value {
            HttpClusterConfig::AwsEks {
                cluster_name,
                region,
            } => Self::AwsEks {
                cluster_name,
                region,
            },
        }
    }
}

impl From<ClusterConfig> for HttpClusterConfig {
    fn from(value: ClusterConfig) -> Self {
        match value {
            ClusterConfig::AwsEks {
                cluster_name,
                region,
            } => Self::AwsEks {
                cluster_name,
                region,
            },
        }
    }
}

/// The body of a [Cluster] creation request.
#[derive(Debug, Clone, Deserialize)]
pub struct CreateClusterHttpRequest {
    cluster_name: String,
    cluster_description: Option<String>,
    cluster_config: HttpClusterConfig,
    credential_id: CredentialId,
}

impl From<CreateClusterHttpRequest> for CreateClusterRequest {
    fn from(value: CreateClusterHttpRequest) -> Self {
        CreateClusterRequest {
            name: value.cluster_name,
            description: value.cluster_description,
            cluster_config: value.cluster_config.into(),
            credential_id: value.credential_id,
        }
    }
}

/// The body of a [Cluster] creation response.
#[derive(Debug, Clone, Serialize)]
pub struct CreateClusterHttpResponse {
    pub cluster_id: ClusterId,
}

/// The body of a [Cluster] get response.
#[derive(Debug, Clone, Serialize)]
pub struct GetClusterHttpResponse {
    pub cluster_id: ClusterId,
    pub cluster_name: String,
    pub cluster_description: Option<String>,
    pub cluster_config: HttpClusterConfig,
    pub credential_id: CredentialId,
}

impl From<Cluster> for GetClusterHttpResponse {
    fn from(value: Cluster) -> Self {
        Self {
            cluster_id: value.id,
            cluster_name: value.name,
            cluster_description: value.description,
            cluster_config: value.cluster_config.into(),
            credential_id: value.credential_id,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct HttpClusterSummary {
    pub cluster_id: ClusterId,
    pub cluster_name: String,
    pub cluster_description: Option<String>,
    pub cluster_type: String,
}

impl From<Cluster> for HttpClusterSummary {
    fn from(cluster: Cluster) -> Self {
        let cluster_type = match cluster.cluster_config {
            ClusterConfig::AwsEks { .. } => "aws_eks".to_string(),
        };

        Self {
            cluster_id: cluster.id,
            cluster_name: cluster.name,
            cluster_description: cluster.description,
            cluster_type,
        }
    }
}

/// The body of a [Cluster] list response.
#[derive(Clone, Debug, Serialize)]
pub struct ListClustersHttpResponse {
    pub clusters: Vec<HttpClusterSummary>,
}

impl From<Vec<Cluster>> for ListClustersHttpResponse {
    fn from(value: Vec<Cluster>) -> Self {
        Self {
            clusters: value.into_iter().map(HttpClusterSummary::from).collect(),
        }
    }
}

/// The body of a [Cluster] connection test request.
#[derive(Debug, Clone, Deserialize)]
pub struct TestClusterHttpRequest {
    pub cluster_config: HttpClusterConfig,
    pub credential_id: CredentialId,
}

#[derive(Clone, Debug, Serialize)]
pub struct TestClusterHttpResponse {
    pub success: bool,
}
