use std::time::{Duration, SystemTime};

use aws_config::{AppName, BehaviorVersion, Region, SdkConfig};
use aws_credential_types::{provider::SharedCredentialsProvider, Credentials};
use aws_sigv4::{
    http_request::{SignableBody, SignableRequest, SignatureLocation, SigningSettings},
    sign::v4::SigningParams,
};
use base64::{
    prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD},
    Engine,
};
use kube::{config::AuthInfo, Config};
use secrecy::{ExposeSecret, SecretString};

use crate::outbound::k8s::{DEFAULT_CONNECT_TIMEOUT, DEFAULT_READ_TIMEOUT, DEFAULT_WRITE_TIMEOUT};

pub struct AwsSigner {
    credentials: Credentials,
    region: String,
}

impl AwsSigner {
    pub fn new(credentials: Credentials, region: String) -> Self {
        Self {
            credentials,
            region: region,
        }
    }

    pub fn get_k8s_token(&self, cluster_id: &str) -> anyhow::Result<SecretString> {
        let credentials = self.credentials.clone().into();
        let mut signing_settings = SigningSettings::default();
        signing_settings.signature_location = SignatureLocation::QueryParams;
        signing_settings.expires_in = Some(Duration::from_secs(60));
        let signing_params = SigningParams::builder()
            .identity(&credentials)
            .region(&self.region)
            .name("sts")
            .time(SystemTime::now())
            .settings(signing_settings)
            .build()
            .unwrap()
            .into();

        let signable_request = SignableRequest::new(
            "GET",
            format!(
                "https://sts.{}.amazonaws.com/?Action=GetCallerIdentity&Version=2011-06-15",
                self.region
            ),
            [("x-k8s-aws-id", cluster_id)].into_iter(),
            SignableBody::Bytes(&[]),
        )
        .expect("signable request");

        let (signing_instructions, _signature) =
            aws_sigv4::http_request::sign(signable_request, &signing_params)
                .unwrap()
                .into_parts();
        let mut request = http::Request::get(format!(
            "https://sts.{}.amazonaws.com/?Action=GetCallerIdentity&Version=2011-06-15",
            self.region
        ))
        .body(http_types::Body::empty())?;
        signing_instructions.apply_to_request_http1x(&mut request);
        let url = request.uri().to_string();
        let b64_url = BASE64_URL_SAFE_NO_PAD.encode(url);
        let token = format!("k8s-aws-v1.{}", b64_url);
        Ok(token.into())
    }
}

pub struct AwsEksAdapter {
    client: aws_sdk_eks::Client,
    signer: AwsSigner,
}

impl AwsEksAdapter {
    pub fn new(access_key: String, secret_key: SecretString, region: Option<String>) -> Self {
        let region = region.unwrap_or("us-east-1".into());
        let credentials = Credentials::from_keys(access_key, secret_key.expose_secret(), None);
        let sdk_config = get_sdk_config(credentials.clone(), region.clone());
        let client = aws_sdk_eks::Client::new(&sdk_config);
        let signer = AwsSigner::new(credentials, region);
        Self { client, signer }
    }

    pub async fn get_eks_kube_config(&self, cluster_name: &str) -> anyhow::Result<Config> {
        let resp = self
            .client
            .describe_cluster()
            .name(cluster_name)
            .send()
            .await?;
        let cluster = resp.cluster.ok_or(anyhow::anyhow!("expected cluster"))?;
        let ca_cert = BASE64_STANDARD.decode(
            cluster
                .certificate_authority
                .ok_or(anyhow::anyhow!("endpoint cluster"))?
                .data
                .ok_or(anyhow::anyhow!("endpoint cluster"))?,
        )?;
        Ok(Config {
            cluster_url: cluster
                .endpoint
                .ok_or(anyhow::anyhow!("endpoint cluster"))?
                .parse()?,
            default_namespace: "default".to_string(),
            auth_info: AuthInfo {
                token: Some(self.signer.get_k8s_token(cluster_name)?),
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

fn get_sdk_config(credentials: Credentials, region: String) -> SdkConfig {
    SdkConfig::builder()
        .app_name(AppName::new("lilac").expect("lilac to be valid app name"))
        .credentials_provider(SharedCredentialsProvider::new(credentials))
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new(region))
        .build()
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

#[cfg(test)]
mod tests {
    use aws_credential_types::Credentials;
    use base64::{prelude::BASE64_URL_SAFE, Engine};
    use secrecy::ExposeSecret;

    use crate::outbound::aws::AwsSigner;

    #[test]
    fn test_sign_k8s_request() {
        let credentials = Credentials::for_tests();
        let signer = AwsSigner::new(credentials, "us-east-1".into());
        let token = signer.get_k8s_token("cluster123").unwrap();
        let (prefix, b64_url) = token.expose_secret().split_once(".").unwrap();
        assert_eq!(prefix, "k8s-aws-v1");
        let decoded_url = BASE64_URL_SAFE.decode(b64_url).unwrap();
        let url = String::from_utf8_lossy(&decoded_url);
        assert!(url.starts_with(
            "https://sts.us-east-1.amazonaws.com/?Action=GetCallerIdentity&Version=2011-06-15"
        ));
    }
}
