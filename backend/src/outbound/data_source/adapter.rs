use async_trait::async_trait;
use secrecy::ExposeSecret;
use snowflake_api::SnowflakeApi;

use crate::domain::dataset::{
    models::{DatasetSource, S3Bucket, Snowflake},
    ports::{DataSourceError, DataSourceTester},
};

pub struct DataSourceImpl;

#[async_trait]
impl DataSourceTester for DataSourceImpl {
    async fn test_connection(&self, source: &DatasetSource) -> Result<(), DataSourceError> {
        match source {
            DatasetSource::S3(s3_bucket) => self.test_s3_connection(s3_bucket).await,
            DatasetSource::Snowflake(snowflake) => self.test_snowflake_connection(snowflake).await,
        }
    }
}

impl DataSourceImpl {
    async fn test_s3_connection(&self, s3_bucket: &S3Bucket) -> Result<(), DataSourceError> {
        // In a real application, we would use the AWS SDK to check if the bucket exists
        // and if we have the necessary permissions to access it.
        let config = aws_config::load_from_env().await;
        let client = aws_sdk_s3::Client::new(&config);
        client
            .head_bucket()
            .bucket(s3_bucket.bucket.clone())
            .send()
            .await
            .map_err(|e| DataSourceError::InvalidConnection(e.to_string()))?;
        Ok(())
    }

    async fn test_snowflake_connection(
        &self,
        snowflake: &Snowflake,
    ) -> Result<(), DataSourceError> {
        // In a real application, we would use the Snowflake driver to check if we can
        // connect to the database and if the schema and table exist.
        let api = SnowflakeApi::with_password_auth(
            &snowflake.account,
            snowflake.warehouse.as_deref(),
            snowflake.database.as_deref(),
            snowflake.schema.as_deref(),
            &snowflake.username,
            snowflake.role.as_deref(),
            snowflake.password.expose_secret(),
        )
        .map_err(|e| DataSourceError::InvalidConnection(e.to_string()))?;

        api.exec("SELECT 1")
            .await
            .map_err(|e| DataSourceError::InvalidConnection(e.to_string()))?;

        Ok(())
    }
}
