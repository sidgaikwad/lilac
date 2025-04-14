//! src/pipeline_definition.rs
//! Defines structures for representing a complete pipeline definition,
//! including data sources and destinations used
//! during the pipeline building phase.

use crate::pipe_core::ImagePipe;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataSource {
    LocalPath(PathBuf),
    // Example: S3Bucket { bucket: String, prefix: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataDestination {
    LocalPath(PathBuf),
    // Example: S3Bucket { bucket: String, prefix: String },
}

#[derive(Clone)]
pub struct Pipeline {
    pub id: String,
    pub name: String,
    pub input_source: DataSource,
    pub output_destination: DataDestination,
    pub stages: Vec<Arc<dyn ImagePipe>>,
}