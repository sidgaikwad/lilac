//! Defines the core data structures, traits, and types used by the image processing pipeline.
//! Version 3: Pipes run as async tasks, connected by channels for streaming processing.

use async_trait::async_trait;
use image::{DynamicImage, ImageFormat};
use std::collections::HashMap;
use std::fmt::Debug;

// --- Basic Data Structures (largely unchanged) ---

pub type ImageMetadata = HashMap<String, serde_json::Value>;

#[derive(Clone, Debug)]
pub struct PipeImageData {
    pub id: String,
    pub image: DynamicImage,
    pub metadata: ImageMetadata,
    pub original_format: ImageFormat,
}

// Representing errors as simple strings for now
pub type PipeError = String;

#[async_trait]
pub trait ImagePipe {
    /// Returns a descriptive name for the pipe.
    fn name(&self) -> &'static str;

    /// Returns a descriptive name for the pipe.
    fn param_definitions(&self) -> Vec<serde_json::Value>;

    /// Runs the pipe stage
    async fn run_stage(
        &self,
        image_batch: Vec<PipeImageData>,
    ) -> Result<Vec<PipeImageData>, PipeError>;
}
