use base64::{prelude::BASE64_STANDARD, Engine};
use google_cloud_auth::credentials::Credentials;
use google_cloud_container_v1::client::ClusterManager;
use kube::{config::AuthInfo, Config};

use crate::{
    domain::credentials::models::GoogleCredentials,
    outbound::k8s::{DEFAULT_CONNECT_TIMEOUT, DEFAULT_READ_TIMEOUT, DEFAULT_WRITE_TIMEOUT},
};

mod token;

pub struct GkeAdapter {
    credentials_client: token::CredentialsClient,
    client: ClusterManager,
}

impl GkeAdapter {
    pub async fn new(google_credentials: GoogleCredentials) -> anyhow::Result<Self> {
        let client = ClusterManager::builder()
            .with_credentials(Credentials::try_from(google_credentials.clone())?)
            .build()
            .await?;
        let credentials_client = token::CredentialsClient::new(google_credentials);
        Ok(Self {
            client,
            credentials_client,
        })
    }

    pub async fn get_gke_kube_config(
        &self,
        project_id: &str,
        location: &str,
        cluster_name: &str,
        namespace: Option<String>,
    ) -> anyhow::Result<Config> {
        let cluster = self
            .client
            .get_cluster()
            .set_name(format!(
                "projects/{project_id}/locations/{location}/clusters/{cluster_name}"
            ))
            .send()
            .await?;
        let ca_cert = BASE64_STANDARD.decode(
            cluster
                .master_auth
                .ok_or(anyhow::anyhow!("endpoint master_auth"))?
                .cluster_ca_certificate,
        )?;
        let token = self
            .credentials_client
            .request_token(&vec![
                "https://www.googleapis.com/auth/cloud-platform".to_string()
            ])
            .await?;
        Ok(Config {
            cluster_url: format!(
                "https://{}",
                cluster
                    .control_plane_endpoints_config
                    .ok_or(anyhow::anyhow!("endpoint control_plane_endpoints_config"))?
                    .ip_endpoints_config
                    .ok_or(anyhow::anyhow!("endpoint ip_endpoints_config"))?
                    .public_endpoint
            )
            .parse()?,
            default_namespace: namespace.unwrap_or("default".to_string()),
            auth_info: AuthInfo {
                token: Some(token.access_token.into()),
                ..Default::default()
            },
            root_cert: Some(parse_certs(&ca_cert)?),
            connect_timeout: Some(DEFAULT_CONNECT_TIMEOUT),
            read_timeout: Some(DEFAULT_READ_TIMEOUT),
            write_timeout: Some(DEFAULT_WRITE_TIMEOUT),
            accept_invalid_certs: false,
            disable_compression: false,
            proxy_url: None,
            tls_server_name: None,
            headers: vec![],
        })
    }
}

fn parse_certs(data: &[u8]) -> Result<Vec<Vec<u8>>, pem::PemError> {
    Ok(pem::parse_many(data)?
        .into_iter()
        .filter_map(|p| {
            if p.tag() == "CERTIFICATE" {
                Some(p.into_contents())
            } else {
                None
            }
        })
        .collect::<Vec<_>>())
}
