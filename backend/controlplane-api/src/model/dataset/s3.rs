use crate::serialize_secret;
use secrecy::SecretString;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, getset::Getters, getset::Setters)]
#[getset(get = "pub", set = "pub")]
pub struct S3Bucket {
    bucket_name: String,
    region: String,
    access_key: String,
    #[serde(serialize_with = "serialize_secret")]
    secret_key: SecretString,
}

impl S3Bucket {
    pub fn new(
        bucket_name: String,
        access_key: String,
        secret_key: SecretString,
        region: String,
    ) -> Self {
        Self {
            bucket_name,
            region,
            access_key,
            secret_key,
        }
    }
}
