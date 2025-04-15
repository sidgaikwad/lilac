//! src/pipe_core.rs
//! Defines the core data structures (like PipeImageData), error types,
//! configuration structure for pipeline stages (PipelineStageConfig),
//! and the central ImagePipe trait that all processing pipes must implement.

use async_trait::async_trait;
use common::model::step_definition::StepDefinition;
use image::{DynamicImage, ImageFormat};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::Debug;
use thiserror::Error;

pub type ImageMetadata = HashMap<String, Value>;

#[derive(Clone, Debug)]
pub struct PipeImageData {
    pub id: String,
    pub image: DynamicImage,
    pub metadata: ImageMetadata,
    pub original_format: ImageFormat,
}

#[derive(Error, Debug, Clone)]
#[error("Pipe Error in {pipe_name}: {message}")]
pub struct PipeError {
    pub pipe_name: String,
    pub message: String,
}

//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineStageConfig {
    pub pipe_identifier: String,
    #[serde(default)]
    pub parameters: HashMap<String, Value>,
}

#[async_trait]
pub trait ImagePipe: Send + Sync {
    fn name(&self) -> &'static str;
    async fn run_stage(
        &self,
        image_batch: Vec<PipeImageData>,
    ) -> Result<Vec<PipeImageData>, PipeError>;
}

pub trait PipeDefinition {
    fn step_definition() -> StepDefinition;
}
