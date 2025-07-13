use aws_config::{AppName, BehaviorVersion, Region, SdkConfig};
use aws_sdk_s3::{
    config::{Credentials, SharedCredentialsProvider},
    operation::head_bucket::HeadBucketError,
    types::BucketLocationConstraint,
    Client,
};
use secrecy::{ExposeSecret, SecretString};

use crate::{model::dataset::S3Bucket, ServiceError};

fn get_s3_client(access_key: &str, secret_key: &SecretString, region: Region) -> Client {
    Client::new(
        &SdkConfig::builder()
            .app_name(AppName::new("lilac").expect("app name to be valid"))
            .behavior_version(BehaviorVersion::latest())
            .credentials_provider(SharedCredentialsProvider::new(
                Credentials::builder()
                    .access_key_id(access_key)
                    .secret_access_key(secret_key.expose_secret())
                    .provider_name("static")
                    .build(),
            ))
            .region(region)
            .build(),
    )
}

pub async fn get_bucket_location(
    bucket_name: &str,
    access_key: &str,
    secret_key: &SecretString,
) -> Result<Region, ServiceError> {
    let client = get_s3_client(access_key, secret_key, Region::from_static("us-east-1"));

    let resp = client
        .get_bucket_location()
        .bucket(bucket_name)
        .send()
        .await;
    match resp {
        Ok(output) => Ok(match output.location_constraint {
            Some(location) => match location {
                BucketLocationConstraint::Eu => Region::from_static("eu-west-1"),
                _ => Region::new(location.to_string()),
            },
            None => Region::from_static("us-east-1"),
        }),
        Err(err) => {
            tracing::error!(error = ?err, bucket_name = %bucket_name, "could not determine bucket location");
            Err(ServiceError::InvalidDataSource {
                source_type: "S3".into(),
                source_id: bucket_name.into(),
            })
        }
    }
}

pub async fn check_s3_bucket_access(bucket: &S3Bucket) -> Result<(), ServiceError> {
    let client = get_s3_client(
        bucket.access_key(),
        bucket.secret_key(),
        Region::new(bucket.region().clone()),
    );

    let resp = client
        .head_bucket()
        .bucket(bucket.bucket_name())
        .send()
        .await;
    match resp {
        Ok(_) => Ok(()),
        Err(err) => {
            let err = err.into_service_error();
            Err(match err {
                HeadBucketError::NotFound(not_found) => {
                    tracing::error!(error = ?not_found.message.unwrap_or_default(), bucket_name = %bucket.bucket_name(), "bucket not found");
                    ServiceError::EntityNotFound {
                        entity_id: bucket.bucket_name().clone(),
                        entity_type: "bucket".into(),
                    }
                }
                _ => ServiceError::BadRequest(format!(
                    "invalid credentials for bucket {}",
                    bucket.bucket_name()
                )),
            })
        }
    }
}
