use std::sync::Arc;

use aws_config::{sts::AssumeRoleProvider, AppName, BehaviorVersion, Region, SdkConfig};
use aws_sdk_s3::{
    config::SharedCredentialsProvider,
    operation::{
        delete_objects::DeleteObjectsError, get_bucket_location::GetBucketLocationError,
        get_object::GetObjectError, list_objects_v2::ListObjectsV2Error,
        put_object::PutObjectError,
    },
    primitives::ByteStream,
    types::{CommonPrefix, Object, ObjectIdentifier},
    Client as S3Client,
};
use aws_smithy_types_convert::date_time::DateTimeExt;
use cached::{proc_macro::cached, SizedCache};
use tracing::instrument;

use crate::{
    model::{
        dataset::{DatasetFile, DatasetId},
        project::ProjectId,
    },
    ServiceError,
};

#[derive(Debug, thiserror::Error)]
pub enum S3Error {
    #[error("not found: {id}")]
    NotFoundError { id: String },

    #[error("decode error")]
    DecodeError,

    #[error("{0}")]
    ListObjectsError(#[from] ListObjectsV2Error),

    #[error("{0}")]
    PutObjectError(#[from] PutObjectError),

    #[error("{0}")]
    GetObjectError(#[from] GetObjectError),

    #[error("{0}")]
    DeleteObjectsError(#[from] DeleteObjectsError),

    #[error("{0}")]
    GetBucketLocationError(#[from] GetBucketLocationError),
}

impl From<S3Error> for ServiceError {
    fn from(value: S3Error) -> ServiceError {
        match value {
            S3Error::NotFoundError { id } => ServiceError::NotFound { id },
            S3Error::ListObjectsError(_) => ServiceError::UnhandledError,
            S3Error::PutObjectError(_) => ServiceError::UnhandledError,
            S3Error::GetObjectError(_) => ServiceError::UnhandledError,
            S3Error::DecodeError => ServiceError::UnhandledError,
            S3Error::DeleteObjectsError(_) => ServiceError::UnhandledError,
            S3Error::GetBucketLocationError(_) => ServiceError::UnhandledError,
        }
    }
}

#[derive(Clone, Debug)]
pub struct S3Wrapper {
    bucket: String,
    client: Arc<S3Client>,
}

impl S3Wrapper {
    pub fn new(bucket: String, client: S3Client) -> Self {
        Self {
            bucket,
            client: Arc::new(client),
        }
    }

    pub async fn new_from_default(bucket: String) -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .app_name(AppName::new("lilac").expect("valid app name"))
            .region(Region::new("us-west-2"))
            .load()
            .await;
        let client = S3Client::new(&config);
        Self {
            bucket,
            client: Arc::new(client),
        }
    }

    pub fn get_dataset_s3_path(
        &self,
        project_id: &ProjectId,
        dataset_id: &DatasetId,
    ) -> String {
        format!(
            "customer_assets/projects/{}/datasets/{}",
            project_id, dataset_id
        )
    }

    #[instrument(level = "info", skip(self, files), ret, err)]
    pub async fn upload_files(
        &self,
        s3_prefix: &str,
        files: Vec<DatasetFile>,
    ) -> Result<(), S3Error> {
        for file in files {
            self.client
                .put_object()
                .bucket(&self.bucket)
                .key(format!("{}/{}", &s3_prefix, &file.metadata.file_name))
                .content_type(file.metadata.file_type)
                .body(ByteStream::from(file.contents))
                .send()
                .await
                .map_err(|e| {
                    tracing::error!("{:?}", e.as_service_error());
                    e.into_service_error()
                })?;
        }
        Ok(())
    }

    #[instrument(level = "info", skip_all, fields(bucket_name), err)]
    pub async fn list_dataset_prefixes(
        &self,
        client: Option<&S3Client>,
        bucket_name: &str,
        s3_prefix: &str,
        start_after_key: Option<&str>,
    ) -> Result<(Vec<CommonPrefix>, Vec<Object>), S3Error> {
        let client = client.unwrap_or(self.client.as_ref());
        let output = client
            .list_objects_v2()
            .bucket(bucket_name)
            .prefix(s3_prefix)
            .delimiter('/')
            .set_start_after(start_after_key.map(|v| v.into()))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e.as_service_error());
                e.into_service_error()
            })?;
        tracing::info!("{:?}", output);
        Ok((
            output.common_prefixes.unwrap_or_default(),
            output.contents.unwrap_or_default(),
        ))
    }

    #[instrument(level = "info", skip(self), err)]
    pub async fn get_dataset_file(&self, key: &str) -> Result<DatasetFile, S3Error> {
        let resp = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("{:?}", e.as_service_error());
                e.into_service_error()
            })?;

        let file_name = key.split("/").last().unwrap();
        let bytes = resp
            .body
            .collect()
            .await
            .map_err(|_| S3Error::DecodeError)?
            .into_bytes();
        Ok(DatasetFile::new(
            file_name.into(),
            resp.content_type.unwrap(),
            resp.content_length.unwrap(),
            resp.last_modified
                .unwrap()
                .to_chrono_utc()
                .expect("to parse"),
            "".into(),
            bytes.to_vec(),
        ))
    }
    #[instrument(level = "info", skip(self), ret, err)]
    pub async fn delete_folder(&self, folder_path: &str) -> Result<(), S3Error> {
        let mut objects_to_delete: Vec<ObjectIdentifier> = Vec::new();
        let mut continuation_token: Option<String> = None;

        loop {
            let list_output = self
                .client
                .list_objects_v2()
                .bucket(&self.bucket)
                .prefix(folder_path)
                .set_continuation_token(continuation_token)
                .send()
                .await
                .map_err(|e| e.into_service_error())?;

            if let Some(contents) = list_output.contents {
                for object in contents {
                    if let Some(key) = object.key {
                        objects_to_delete.push(
                            ObjectIdentifier::builder()
                                .key(key)
                                .build()
                                .map_err(|_e| S3Error::DecodeError)?,
                        );
                    }
                }
            }

            if list_output.is_truncated.unwrap_or(false) {
                continuation_token = list_output.next_continuation_token;
            } else {
                break;
            }
        }

        if !objects_to_delete.is_empty() {
            // S3 DeleteObjects can take up to 1000 keys at a time.
            for chunk in objects_to_delete.chunks(1000) {
                let delete_request = aws_sdk_s3::types::Delete::builder()
                    .set_objects(Some(Vec::from(chunk)))
                    .quiet(false) // Set to true if you don't want verbose output
                    .build()
                    .map_err(|_e| S3Error::DecodeError)?;

                self.client
                    .delete_objects()
                    .bucket(&self.bucket)
                    .delete(delete_request)
                    .send()
                    .await
                    .map_err(|e| e.into_service_error())?;
            }
        }
        Ok(())
    }

    pub async fn get_bucket_location(&self, bucket_name: &str) -> Result<String, S3Error> {
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

#[cached(
    ty = "SizedCache<String, S3Client>",
    create = "{ SizedCache::with_size(10) }",
    convert = r#"{ format!("{}{}{}", region, role_arn, external_id) }"#
)]
pub async fn get_s3_client_with_role(region: &str, role_arn: &str, external_id: &str) -> S3Client {
    S3Client::new(
        &SdkConfig::builder()
            .behavior_version(BehaviorVersion::latest())
            .app_name(AppName::new("Lilac").expect("AppName to be valid"))
            .region(Region::new(region.to_string()))
            .credentials_provider(SharedCredentialsProvider::new(
                AssumeRoleProvider::builder(role_arn)
                    .external_id(external_id)
                    .region(Region::new(region.to_string()))
                    .session_name("Lilac")
                    .build()
                    .await,
            ))
            .build(),
    )
}
