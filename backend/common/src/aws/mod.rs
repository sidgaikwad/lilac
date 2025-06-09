mod s3;
pub use s3::{S3Error, S3Wrapper, get_s3_client_with_role};
mod sts;
pub use sts::{STSError, STSWrapper};
