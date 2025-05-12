//! src/pipeline_definition.rs
//! Defines structures for representing a complete pipeline definition,
//! including data sources and destinations used
//! during the pipeline building phase.

use crate::pipe_core::ImagePipe;
use common::model::dataset::DatasetId;
use common::s3::S3Wrapper;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum DataSource {
    LocalPath(PathBuf),
    S3Path(S3Wrapper, String),
    // Example: S3Bucket { bucket: String, prefix: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataDestination {
    LocalPath(PathBuf),
    Dataset(DatasetId),
    // Example: S3Bucket { bucket: String, prefix: String },
}

pub struct Pipeline {
    pub id: String,
    pub name: String,
    pub input_source: DataSource,
    pub output_destination: DataDestination,
    pub stages: Vec<Box<dyn ImagePipe>>,
}
