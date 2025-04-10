//! Pipe implementation for standardizing image resolution (Streaming/Channel Processing).
//! Implements the async run_stage method, using channels and spawn_blocking.

use crate::pipeline::{
    self,
    ChannelInput,
    ChannelOutput,
    ImagePipe,
    PipeConfig,
    PipeError,
    PipeImageData,
    SharedPipelineState, // Channel communication types
};
use crate::utils::log_pipe_event;
use async_trait::async_trait;
use image::imageops::{self, FilterType};
use image::{DynamicImage, GenericImageView};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task; // For spawn_blocking

#[derive(Debug)]
pub struct ResolutionStandardizerPipe;

impl ResolutionStandardizerPipe {
    fn get_params(&self, config: &PipeConfig) -> Result<(u32, u32, FilterType), PipeError> {
        let target_width = config
            .parameters
            .get("target_width")
            .and_then(Value::as_u64)
            .map(|v| v as u32)
            .ok_or_else(|| "Missing or invalid 'target_width' parameter".to_string())?;

        let target_height = config
            .parameters
            .get("target_height")
            .and_then(Value::as_u64)
            .map(|v| v as u32)
            .ok_or_else(|| "Missing or invalid 'target_height' parameter".to_string())?;

        let filter_type_str = config
            .parameters
            .get("filter_type")
            .and_then(Value::as_str)
            .unwrap_or("Lanczos3");

        let filter_type = match filter_type_str.to_lowercase().as_str() {
            "nearest" => FilterType::Nearest,
            "triangle" => FilterType::Triangle,
            "catmullrom" => FilterType::CatmullRom,
            "gaussian" => FilterType::Gaussian,
            "lanczos3" => FilterType::Lanczos3,
            _ => FilterType::Lanczos3, // Default on invalid string
        };

        if target_width == 0 || target_height == 0 {
            return Err("Target width and height must be greater than 0".to_string());
        }
        Ok((target_width, target_height, filter_type))
    }
}

#[async_trait]
impl ImagePipe for ResolutionStandardizerPipe {
    fn name(&self) -> &'static str {
        "ResolutionStandardizer"
    }

    /// Runs the resolution standardization stage as an asynchronous task.
    /// Reads from input_rx, processes using spawn_blocking for CPU work,
    /// and sends results to output_tx.
    async fn run_stage(
        self: Arc<Self>,
        config: Arc<PipeConfig>,
        _shared_state: Option<SharedPipelineState>,
        input_rx: &mut mpsc::Receiver<ChannelInput>,
        output_tx: mpsc::Sender<ChannelOutput>,
    ) {
        let pipe_name = self.name();

        // 1. Get parameters once for the stage task
        let params = match self.get_params(&config) {
            Ok(p) => p,
            Err(e) => {
                log_pipe_event(
                    pipe_name,
                    "STAGE_INIT",
                    "ERROR",
                    &format!("Invalid configuration for stage: {}", e),
                );
                return; // Exit task if config is bad
            }
        };
        let (target_width, target_height, filter_type) = params;
        log_pipe_event(
            pipe_name,
            "STAGE_INIT",
            "DEBUG",
            &format!(
                "Stage configured with Target: {}x{}, Filter: {:?}",
                target_width, target_height, filter_type
            ),
        );

        // 2. Process items received from the input channel
        while let Some(input_result) = input_rx.recv().await {
            // Clone image_id *outside* the match for potential error logging later
            // If input_result is Err, we won't have data.id, handle this case.
            let maybe_image_id = match &input_result {
                Ok(data) => Some(data.id.clone()),
                Err(_) => None, // No ID if we received an error
            };

            let output_result: ChannelOutput = match input_result {
                // Explicit type for clarity
                // --- If received valid data, process it ---
                Ok(mut data) => {
                    // We have a valid ID here
                    let image_id = data.id.clone(); // Clone for logging within this block and for move
                    let image_id_for_match = image_id.clone(); // <<-- CLONE HERE for use after await
                    log_pipe_event(
                        pipe_name,
                        &image_id,
                        "DEBUG",
                        "Received item for processing.",
                    );

                    // --- CPU-Bound Work: Move to spawn_blocking ---
                    let pipe_name_clone = pipe_name; // &'static str is Copy

                    let processing_result = task::spawn_blocking(
                        // Add explicit return type annotation for the closure
                        move || -> Result<ChannelOutput, PipeError> {
                            // image_id (original clone) is moved into this closure

                            // Check if resizing is needed
                            let (current_width, current_height) = data.image.dimensions();
                            if current_width == target_width && current_height == target_height {
                                log_pipe_event(
                                    pipe_name_clone,
                                    &image_id,
                                    "INFO",
                                    "Image already target size.",
                                );
                                return Ok(Ok(data));
                            }

                            // Perform resizing
                            log_pipe_event(
                                pipe_name_clone,
                                &image_id,
                                "INFO",
                                &format!(
                                    "Resizing from {}x{} to {}x{}",
                                    current_width, current_height, target_width, target_height
                                ),
                            );
                            let resized_image = imageops::resize(
                                &data.image,
                                target_width,
                                target_height,
                                filter_type,
                            );

                            // Update data and metadata
                            data.image = DynamicImage::ImageRgba8(resized_image);
                            data.metadata
                                .insert("resized_width".to_string(), target_width.into());
                            data.metadata
                                .insert("resized_height".to_string(), target_height.into());
                            data.metadata.insert(
                                "original_width_before_resize".to_string(),
                                current_width.into(),
                            );
                            data.metadata.insert(
                                "original_height_before_resize".to_string(),
                                current_height.into(),
                            );

                            log_pipe_event(pipe_name_clone, &image_id, "INFO", "Resize complete.");
                            Ok(Ok(data))
                        },
                    )
                    .await; // Await the result of spawn_blocking

                    // --- Handle spawn_blocking result ---
                    match processing_result {
                        Ok(Ok(channel_output)) => {
                            // spawn_blocking succeeded and closure succeeded
                            channel_output // This is Result<PipeImageData, PipeError>
                        }
                        Ok(Err(pipe_err)) => {
                            // spawn_blocking succeeded BUT closure returned an Err
                            // Use the clone created before spawn_blocking
                            log_pipe_event(
                                pipe_name,
                                &image_id_for_match,
                                "ERROR",
                                &format!("Processing failed inside blocking task: {}", pipe_err),
                            ); // <-- Use clone
                            Err(pipe_err) // Forward the error
                        }
                        Err(join_err) => {
                            // spawn_blocking task panicked or was cancelled
                            // Use the clone created before spawn_blocking
                            log_pipe_event(
                                pipe_name,
                                &image_id_for_match,
                                "ERROR",
                                &format!("Blocking task failed: {}", join_err),
                            ); // <-- Use clone
                            Err(format!(
                                "Blocking task failed for {}: {}",
                                image_id_for_match, join_err
                            ))
                        }
                    }
                }
                // --- If received an error, forward it ---
                Err(e) => {
                    // Use the maybe_image_id captured before the match
                    let id_str = maybe_image_id.as_deref().unwrap_or("UNKNOWN_ID");
                    log_pipe_event(pipe_name, id_str, "WARN", "Forwarding previous error.");
                    Err(e) // Forward the error to the next stage
                }
            }; // End of match input_result

            // 3. Send the result to the output channel
            if output_tx.send(output_result).await.is_err() {
                // Use maybe_image_id for logging if available
                let id_str = maybe_image_id.as_deref().unwrap_or("STAGE_EXIT");
                log_pipe_event(
                    pipe_name,
                    id_str,
                    "WARN",
                    "Output channel receiver dropped. Exiting stage task.",
                );
                break; // Exit the while loop
            }
        } // End of while let Some

        log_pipe_event(
            pipe_name,
            "STAGE_EXIT",
            "INFO",
            "Input channel closed. Exiting stage task.",
        );
    } // End of run_stage
}
