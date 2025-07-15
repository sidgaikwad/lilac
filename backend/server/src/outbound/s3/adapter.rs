use async_trait::async_trait;
use aws_config::{AppName, BehaviorVersion, Region};
use aws_sdk_s3::Client as S3Client;

use crate::domain::integration::ports::S3Port;


#[derive(Clone, Debug)]
pub struct S3Adapter {
    client: S3Client,
}

impl S3Adapter {
    pub async fn new() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .app_name(AppName::new("lilac").expect("valid app name"))
            .region(Region::new("us-west-2"))
            .load()
            .await;
        let client = S3Client::new(&config);
        Self { client }
    }
}

#[async_trait]
impl S3Port for S3Adapter {
    async fn get_bucket_location(&self, bucket_name: &str) -> Result<String, anyhow::Error> {
        let resp = self
            .client
            .get_bucket_location()
            .bucket(bucket_name)
            .send()
            .await
            .map_err(|e| e.into_service_error())?;

        Ok(resp
            .location_constraint
            .map(|v| v.to_string())
            .unwrap_or_default())
    }
}