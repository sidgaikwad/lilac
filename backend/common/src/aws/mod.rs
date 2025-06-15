mod s3;
pub use s3::{get_s3_client_with_role, S3Error, S3Wrapper};
mod sts;
pub use sts::{STSError, STSWrapper};
