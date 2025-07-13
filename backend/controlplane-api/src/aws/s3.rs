use std::sync::Arc;

use aws_config::{AppName, BehaviorVersion, Region};
use aws_sdk_s3::{
    operation::{
        delete_objects::DeleteObjectsError, get_bucket_location::GetBucketLocationError,
        get_object::GetObjectError, list_objects_v2::ListObjectsV2Error,
        put_object::PutObjectError,
    },
    Client as S3Client,
};

use crate::{
    model::{dataset::DatasetId, project::ProjectId},
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
            S3Error::NotFoundError { id } => ServiceError::EntityNotFound {
                entity_type: "bucket".into(),
                entity_id: id,
            },
            S3Error::ListObjectsError(err) => ServiceError::Unhandled(Box::new(err)),
            S3Error::PutObjectError(err) => ServiceError::Unhandled(Box::new(err)),
            S3Error::GetObjectError(err) => ServiceError::Unhandled(Box::new(err)),
            S3Error::DecodeError => ServiceError::InternalError("s3 decode error".into()),
            S3Error::DeleteObjectsError(err) => ServiceError::Unhandled(Box::new(err)),
            S3Error::GetBucketLocationError(err) => ServiceError::Unhandled(Box::new(err)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct S3Wrapper {
    client: Arc<S3Client>,
}

impl S3Wrapper {
    pub fn new(client: S3Client) -> Self {
        Self {
            client: Arc::new(client),
        }
    }

    pub async fn new_from_default() -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest())
            .app_name(AppName::new("lilac").expect("valid app name"))
            .region(Region::new("us-west-2"))
            .load()
            .await;
        let client = S3Client::new(&config);
        Self {
            client: Arc::new(client),
        }
    }

    pub fn get_dataset_s3_path(&self, project_id: &ProjectId, dataset_id: &DatasetId) -> String {
        format!(
            "customer_assets/projects/{project_id}/datasets/{dataset_id}"
        )
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
