use std::sync::Arc;

use aws_config::{AppName, BehaviorVersion, Region, SdkConfig};
use aws_sdk_s3::{
    operation::{
        delete_objects::DeleteObjectsError, get_object::GetObjectError,
        list_objects_v2::ListObjectsV2Error, put_object::PutObjectError,
    },
    primitives::ByteStream,
    types::ObjectIdentifier,
    Client as S3Client,
};
use aws_smithy_types_convert::date_time::DateTimeExt;
use tracing::instrument;

use crate::{
    model::{
        dataset::{DatasetFile, DatasetFileMetadata, DatasetId},
        organization::OrganizationId,
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
            .app_name(AppName::new("data-project").expect("valid app name"))
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
        org_id: &OrganizationId,
        project_id: &ProjectId,
        dataset_id: &DatasetId,
    ) -> String {
        format!(
            "customer_assets/organizations/{}/projects/{}/datasets/{}",
            org_id, project_id, dataset_id
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

    #[instrument(level = "info", skip(self), ret, err)]
    pub async fn list_dataset_files(
        &self,
        s3_prefix: &str,
    ) -> Result<Vec<DatasetFileMetadata>, S3Error> {
        let mut stream = self
            .client
            .list_objects_v2()
            .bucket(&self.bucket)
            .prefix(s3_prefix)
            .into_paginator()
            .send();

        const BUCKET_REGION: &str = "us-west-2";
        let mut files = Vec::new();
        while let Some(page) = stream.next().await {
            let output = page.map_err(|e| e.into_service_error())?;
            match output.contents {
                Some(contents) => {
                    files.extend(contents.into_iter().filter_map(|v| {
                        Some(DatasetFileMetadata::new(
                            v.key.clone()?.strip_prefix(s3_prefix)?.to_string(),
                            "".to_string(),
                            v.size?,
                            v.last_modified?.to_chrono_utc().ok()?,
                            format!("https://{}.s3.{}.amazonaws.com/{}", &self.bucket, BUCKET_REGION, &v.key?),
                        ))
                    }));
                }
                None => continue,
            }
        }
        Ok(files)
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
                        objects_to_delete.push(ObjectIdentifier::builder().key(key).build().map_err(|_e| S3Error::DecodeError)?);
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
                    .build().map_err(|_e| S3Error::DecodeError)?;

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
}
