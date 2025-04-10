//! Pipe implementation for detecting blurry images using configurable methods.
//! Methods: LaplacianVariance, EdgeIntensity (Sobel Mean), PixelVariance, EdgeCount (Sobel).
//! Implements async streaming stage processing using channels and spawn_blocking.

use crate::pipeline::{
    ChannelInput, ChannelOutput, ImagePipe, PipeConfig, PipeError, PipeImageData,
    SharedPipelineState,
};
use crate::utils::log_pipe_event;
use async_trait::async_trait;
use image::{GrayImage, Luma};
use imageproc::{filter, gradients};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::task; // For spawn_blocking

/// Enum to represent the chosen blur detection method.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BlurDetectionMethod {
    LaplacianVariance,
    EdgeIntensity, // Mean of Sobel gradient magnitudes
    PixelVariance, // Variance of grayscale pixel intensities
    EdgeCount,     // Count of Sobel gradient magnitudes above a threshold
}

/// Parameters extracted from the config, specific to the chosen method.
#[derive(Debug, Clone, Copy)] // Needs Clone + Copy for spawn_blocking closure
struct BlurParams {
    method: BlurDetectionMethod,
    threshold: f64,
    // Only used for EdgeCount method
    edge_magnitude_threshold: Option<u16>,
}

/// A pipe that detects blurry images based on a configured method and threshold.
#[derive(Debug)]
pub struct BlurDetectorPipe;

// Helper functions remain synchronous and largely unchanged
impl BlurDetectorPipe {
    /// Extracts parameters: detection method and the relevant thresholds.
    fn get_params(&self, config: &PipeConfig) -> Result<BlurParams, PipeError> {
        let method_str = config
            .parameters
            .get("detection_method")
            .and_then(Value::as_str)
            .unwrap_or("laplacian_variance");

        let method = match method_str.to_lowercase().as_str() {
            "laplacian_variance" => BlurDetectionMethod::LaplacianVariance,
            "edge_intensity" => BlurDetectionMethod::EdgeIntensity,
            "pixel_variance" => BlurDetectionMethod::PixelVariance,
            "edge_count" => BlurDetectionMethod::EdgeCount,
            _ => return Err(format!("Unknown detection_method: {}", method_str)),
        };

        let threshold_key = match method {
            BlurDetectionMethod::LaplacianVariance => "laplacian_threshold",
            BlurDetectionMethod::EdgeIntensity => "edge_intensity_threshold",
            BlurDetectionMethod::PixelVariance => "pixel_variance_threshold",
            BlurDetectionMethod::EdgeCount => "edge_count_threshold",
        };

        let threshold = config
            .parameters
            .get(threshold_key)
            .and_then(Value::as_f64)
            .ok_or_else(|| format!("Missing or invalid '{}' parameter", threshold_key))?;

        if threshold < 0.0 {
            return Err(format!("{} cannot be negative", threshold_key));
        }

        let mut edge_magnitude_threshold: Option<u16> = None;
        if method == BlurDetectionMethod::EdgeCount {
            let mag_threshold = config
                .parameters
                .get("edge_magnitude_threshold")
                .and_then(Value::as_u64)
                .map(|v| v as u16)
                .ok_or_else(|| "Missing 'edge_magnitude_threshold' for EdgeCount".to_string())?;
            edge_magnitude_threshold = Some(mag_threshold);
        }

        Ok(BlurParams {
            method,
            threshold,
            edge_magnitude_threshold,
        })
    }

    // --- Calculation Helpers (Remain Synchronous) ---
    fn calculate_laplacian_variance(
        laplacian_img: &image::ImageBuffer<Luma<i16>, Vec<i16>>,
    ) -> f64 {
        let count = laplacian_img.pixels().len() as f64;
        if count == 0.0 {
            return 0.0;
        }
        let mut sum = 0.0;
        let mut sum_sq = 0.0;
        for pixel_val in laplacian_img.pixels().map(|p| p[0] as f64) {
            sum += pixel_val;
            sum_sq += pixel_val * pixel_val;
        }
        let mean = sum / count;
        (sum_sq / count) - (mean * mean)
    }
    fn calculate_mean_gradient_intensity(
        gradient_img: &image::ImageBuffer<Luma<u16>, Vec<u16>>,
    ) -> f64 {
        let count = gradient_img.pixels().len() as f64;
        if count == 0.0 {
            return 0.0;
        }
        let mut sum = 0.0;
        for pixel_val in gradient_img.pixels().map(|p| p[0] as f64) {
            sum += pixel_val;
        }
        sum / count
    }
    fn calculate_pixel_variance(gray_img: &GrayImage) -> f64 {
        let count = gray_img.pixels().len() as f64;
        if count == 0.0 {
            return 0.0;
        }
        let mut sum = 0.0;
        let mut sum_sq = 0.0;
        for pixel_val in gray_img.pixels().map(|p| p[0] as f64) {
            sum += pixel_val;
            sum_sq += pixel_val * pixel_val;
        }
        let mean = sum / count;
        (sum_sq / count) - (mean * mean)
    }
    fn calculate_edge_count(
        gradient_img: &image::ImageBuffer<Luma<u16>, Vec<u16>>,
        magnitude_threshold: u16,
    ) -> f64 {
        let mut edge_pixel_count = 0;
        for pixel_val in gradient_img.pixels().map(|p| p[0]) {
            if pixel_val >= magnitude_threshold {
                edge_pixel_count += 1;
            }
        }
        edge_pixel_count as f64
    }
}

#[async_trait]
impl ImagePipe for BlurDetectorPipe {
    fn name(&self) -> &'static str {
        "BlurDetector"
    }

    /// Runs the blur detection stage as an asynchronous task.
    async fn run_stage(
        self: Arc<Self>,
        config: Arc<PipeConfig>,
        _shared_state: Option<SharedPipelineState>, // Parameter included but not used by this pipe
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
        log_pipe_event(
            pipe_name,
            "STAGE_INIT",
            "DEBUG",
            &format!("Stage configured with params: {:?}", params),
        );

        // 2. Process items received from the input channel
        while let Some(input_result) = input_rx.recv().await {
            let maybe_image_id = match &input_result {
                Ok(data) => Some(data.id.clone()),
                Err(_) => None,
            };

            let output_result: Option<ChannelOutput> = match input_result {
                // Output is optional (for discards)
                Ok(data) => {
                    let image_id = data.id.clone();
                    let image_id_for_match = image_id.clone(); // Clone for use after await
                    log_pipe_event(
                        pipe_name,
                        &image_id,
                        "DEBUG",
                        "Received item for processing.",
                    );

                    // --- CPU-Bound Work: Move to spawn_blocking ---
                    let pipe_name_clone = pipe_name;

                    // Closure returns Result<Option<PipeImageData>, PipeError>
                    // Ok(Some(data)) = Unchanged/Modified
                    // Ok(None) = Discarded
                    // Err(pipe_error) = Error during processing
                    let processing_result = task::spawn_blocking(
                        move || -> Result<Option<PipeImageData>, PipeError> {
                            // data and params are moved into closure
                            let mut current_data = data; // Make mutable for metadata insertion

                            // Convert to grayscale
                            let gray_image = current_data.image.to_luma8();

                            // Calculate blur metric
                            let (metric_name, metric_value) = match params.method {
                                BlurDetectionMethod::LaplacianVariance => {
                                    let laplacian_image = filter::laplacian_filter(&gray_image);
                                    let variance =
                                        Self::calculate_laplacian_variance(&laplacian_image);
                                    ("laplacian_variance", variance)
                                }
                                BlurDetectionMethod::EdgeIntensity => {
                                    let gradients = gradients::sobel_gradients(&gray_image);
                                    let mean_intensity =
                                        Self::calculate_mean_gradient_intensity(&gradients);
                                    ("edge_intensity_mean", mean_intensity)
                                }
                                BlurDetectionMethod::PixelVariance => {
                                    let variance = Self::calculate_pixel_variance(&gray_image);
                                    ("pixel_variance", variance)
                                }
                                BlurDetectionMethod::EdgeCount => {
                                    let mag_threshold = params.edge_magnitude_threshold.unwrap();
                                    let gradients = gradients::sobel_gradients(&gray_image);
                                    let count =
                                        Self::calculate_edge_count(&gradients, mag_threshold);
                                    ("edge_count", count)
                                }
                            };

                            log_pipe_event(
                                pipe_name_clone,
                                &image_id,
                                "INFO",
                                &format!("Calculated {}: {:.2}", metric_name, metric_value),
                            );

                            // Add metadata
                            current_data
                                .metadata
                                .insert(metric_name.to_string(), metric_value.into());
                            current_data.metadata.insert(
                                "blur_detection_method".to_string(),
                                format!("{:?}", params.method).to_lowercase().into(),
                            );
                            current_data.metadata.insert(
                                format!("{}_threshold", metric_name),
                                params.threshold.into(),
                            );
                            if let Some(mag_thresh) = params.edge_magnitude_threshold {
                                current_data.metadata.insert(
                                    "edge_magnitude_threshold".to_string(),
                                    mag_thresh.into(),
                                );
                            }

                            // Compare metric and decide: Ok(Some(data)) or Ok(None) for discard
                            if metric_value < params.threshold {
                                let reason = format!(
                                    "{} {:.2} is below threshold {}",
                                    metric_name, metric_value, params.threshold
                                );
                                log_pipe_event(
                                    pipe_name_clone,
                                    &image_id,
                                    "INFO",
                                    &format!("Discarding image: {}", reason),
                                );
                                Ok(None) // Signal discard
                            } else {
                                log_pipe_event(
                                    pipe_name_clone,
                                    &image_id,
                                    "INFO",
                                    &format!("Image passed {} check.", metric_name),
                                );
                                Ok(Some(current_data)) // Signal pass (unchanged data content)
                            }
                        }, // End of closure
                    )
                    .await; // Await spawn_blocking

                    // --- Handle spawn_blocking result ---
                    match processing_result {
                        Ok(Ok(Some(processed_data))) => Some(Ok(processed_data)), // Pass
                        Ok(Ok(None)) => None, // Discard - send nothing
                        Ok(Err(pipe_err)) => {
                            // Logical error within closure
                            log_pipe_event(
                                pipe_name,
                                &image_id_for_match,
                                "ERROR",
                                &format!("Processing failed inside blocking task: {}", pipe_err),
                            );
                            Some(Err(pipe_err)) // Send error result
                        }
                        Err(join_err) => {
                            // spawn_blocking task failed (panic/cancel)
                            log_pipe_event(
                                pipe_name,
                                &image_id_for_match,
                                "ERROR",
                                &format!("Blocking task failed: {}", join_err),
                            );
                            Some(Err(format!(
                                "Blocking task failed for {}: {}",
                                image_id_for_match, join_err
                            ))) // Send error result
                        }
                    }
                }
                // --- If received an error, forward it ---
                Err(e) => {
                    let id_str = maybe_image_id.as_deref().unwrap_or("UNKNOWN_ID");
                    log_pipe_event(pipe_name, id_str, "WARN", "Forwarding previous error.");
                    Some(Err(e)) // Forward the error wrapped in Some for consistency
                }
            }; // End of match input_result

            // 3. Send the result to the output channel if it wasn't a discard
            if let Some(result_to_send) = output_result {
                if output_tx.send(result_to_send).await.is_err() {
                    let id_str = maybe_image_id.as_deref().unwrap_or("STAGE_EXIT");
                    log_pipe_event(
                        pipe_name,
                        id_str,
                        "WARN",
                        "Output channel receiver dropped. Exiting stage task.",
                    );
                    break; // Exit the while loop
                }
            }
        }

        log_pipe_event(
            pipe_name,
            "STAGE_EXIT",
            "INFO",
            "Input channel closed. Exiting stage task.",
        );
    } // End of run_stage
}
