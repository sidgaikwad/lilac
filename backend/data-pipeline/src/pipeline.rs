//! Defines the core data structures, traits, and types used by the image processing pipeline.
//! Version 3: Pipes run as async tasks, connected by channels for streaming processing.

use async_trait::async_trait;
use image::{DynamicImage, ImageFormat};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc; // Needed for sharing pipes/configs across tasks
use tokio::sync::mpsc; // For asynchronous channels

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

// Item type flowing between pipeline stages via channels
// Using Result allows propagating errors gracefully.
pub type ChannelInput = Result<PipeImageData, PipeError>;
// Output type is the same - result of the current stage
pub type ChannelOutput = Result<PipeImageData, PipeError>;

#[derive(Deserialize, Clone, Debug)]
pub struct PipeConfig {
    #[serde(default)]
    pub parameters: HashMap<String, serde_json::Value>,
}

// Type alias for shared state (e.g., for deduplication) - kept definition for future use
// Needs Arc<Mutex<...>> for safe sharing across async tasks.
pub type SharedPipelineState = Arc<tokio::sync::Mutex<HashMap<String, bool>>>;

// --- The core trait - MODIFIED FOR ASYNC STREAMING/CHANNELS ---

#[async_trait]
pub trait ImagePipe: Send + Sync {
    // Send + Sync needed because Arc<Self> will be sent across task boundaries.

    /// Returns a descriptive name for the pipe.
    fn name(&self) -> &'static str;

    /// Runs the pipeline stage as an asynchronous task.
    /// Reads from input_rx, processes (using spawn_blocking for CPU work),
    /// and sends results to output_tx.
    /// Takes Arc<Self> and Arc<PipeConfig> for sharing across tasks.
    async fn run_stage(
        self: Arc<Self>,         // Use Arc<Self> to allow sharing the pipe instance
        config: Arc<PipeConfig>, // Use Arc<PipeConfig>
        shared_state: Option<SharedPipelineState>, // Pass shared state if needed
        input_rx: &mut mpsc::Receiver<ChannelInput>, // Mutable borrow of receiver
        output_tx: mpsc::Sender<ChannelOutput>, // Sender to next stage
    );

    // Optional helper for the common pattern within run_stage?
    // Or maybe the logic lives entirely within run_stage implementation.
    // async fn process_single_item(&self, data: PipeImageData, config: &PipeConfig) -> Result<PipeImageData, PipeError>;
}
