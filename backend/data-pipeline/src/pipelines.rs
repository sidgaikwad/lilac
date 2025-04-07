// The core data structures, traits, and types for the image processing pipeline
use async_trait::async_trait;
use image::{DynamicImage, ImageFormat};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug; // Import Debug trait
use std::sync::Arc;
use tokio::sync::Mutex;


pub type ImageMetadata = HashMap<String, serde_json::Value>;

#[derive(Clone, Debug)]
pub struct PipeImageData {
    pub id: String,
    pub image: DynamicImage,
    pub metadata: ImageMetadata,
    pub original_format: ImageFormat,
}

#[derive(Debug)]
pub enum PipeResult {
    Modified(PipeImageData), // Image was successfully processed and modified
    Unchanged(PipeImageData), // Image was successfully process but not modified
    Discarded { reason: String}, // Image was filtered out
    Error {message: String}, // Error
}

#[derive(Deserialize, Clone, Debug)]
pub struct PipeConfig {
    // Key-Value parameters specific to a pipe's operation
    #[serde(default)] // Use default HashMap::new if 'parameters' is missing
    pub parameters: HashMap<String, serde_json::Value>,
}

pub type SharedPipelineState = Arc<Mutex<HashMap<String, bool>>>;

// The core trait that all image processing pipes must implement.
#[async_trait]
pub trait ImagePipe: Send + Sync {
    /// Returns a descriptive name for the pipe (used for logging, UI).
    fn name(&self) -> &'static str;

    /// The main processing function for the pipe.
    /// It takes the image data, its own configuration, and optionally the shared pipeline state.
    /// It returns a PipeResult indicating the outcome.
    /// This function is async to allow for potentially long-running operations
    /// (like I/O or complex computations) without blocking the executor.
    async fn process(
        &self,
        data: PipeImageData,
        config: &PipeConfig,
        shared_state: Option<&SharedPipelineState>, // Only provide if the pipe needs state
    ) -> PipeResult;
}