//! Pipe implementation for detecting blurry images using configurable methods.
//! Methods: LaplacianVariance, EdgeIntensity (Sobel Mean), PixelVariance, EdgeCount (Sobel).
//! Implements async streaming stage processing using channels and spawn_blocking.

use crate::pipe_core::{ImagePipe, PipeError, PipeImageData};
use crate::utils::log_pipe_event;
use async_trait::async_trait;
use image::{DynamicImage, GrayImage, Luma};
use imageproc::{filter, gradients};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_json::json;

/// Enum to represent the chosen blur detection method.
#[derive(Debug, PartialEq, Eq, Clone, Copy, strum::Display)]
pub enum BlurDetectionMethod {
    LaplacianVariance,
    EdgeIntensity,                               // Mean of Sobel gradient magnitudes
    PixelVariance,                               // Variance of grayscale pixel intensities
    EdgeCount { edge_magnitude_threshold: u16 }, // Count of Sobel gradient magnitudes above a threshold
}

/// A pipe that detects blurry images based on a configured method and threshold.
#[derive(Debug)]
pub struct BlurDetectorPipe {
    method: BlurDetectionMethod,
    threshold: f64,
}

// Helper functions remain synchronous and largely unchanged
impl BlurDetectorPipe {
    pub fn new(method: BlurDetectionMethod, threshold: f64) -> Self {
        Self { method, threshold }
    }

    fn calculate_blur(&self, img: DynamicImage) -> f64 {
        let gray_image = img.into_luma8();
        match self.method {
            BlurDetectionMethod::LaplacianVariance => {
                let laplacian_image = filter::laplacian_filter(&gray_image);
                self.calculate_laplacian_variance(&laplacian_image)
            }
            BlurDetectionMethod::EdgeIntensity => {
                let gradients = gradients::sobel_gradients(&gray_image);
                self.calculate_mean_gradient_intensity(&gradients)
            }
            BlurDetectionMethod::PixelVariance => self.calculate_pixel_variance(&gray_image),
            BlurDetectionMethod::EdgeCount {
                edge_magnitude_threshold,
            } => {
                let gradients = gradients::sobel_gradients(&gray_image);
                self.calculate_edge_count(&gradients, edge_magnitude_threshold)
            }
        }
    }

    // --- Calculation Helpers (Remain Synchronous) ---
    fn calculate_laplacian_variance(
        &self,
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
        &self,
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
    fn calculate_pixel_variance(&self, gray_img: &GrayImage) -> f64 {
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
        &self,
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

    fn param_definitions(&self) -> Vec<serde_json::Value> {
        // TODO: return a proper jsonschema instead
        vec![
            json!({
                "parameter_name": "method",
                "parameter_type": "enum"
            }),
            json!({
                "parameter_name": "threshold",
                "parameter_type": "float"
            }),
            json!({
                "parameter_name": "method_params",
                "parameter_type": "enum"
            }),
        ]
    }

    /// Runs the blur detection stage as an asynchronous task.
    async fn run_stage(
        &self,
        image_batch: Vec<PipeImageData>,
    ) -> Result<Vec<PipeImageData>, PipeError> {
        let pipe_name = self.name();

        log_pipe_event(
            pipe_name,
            "STAGE_INIT",
            "DEBUG",
            &format!(
                "Stage configured with params: {:?} {:?}",
                self.method, self.threshold
            ),
        );

        // 2. Process items received from the input channel

        let output = image_batch
            .into_par_iter()
            .filter(|img| {
                log_pipe_event(pipe_name, &img.id, "DEBUG", "Received item for processing.");
                // TODO: avoid cloning
                let blur = self.calculate_blur(img.image.clone());

                log_pipe_event(
                    &pipe_name,
                    &img.id,
                    "INFO",
                    &format!("Calculated {}: {:.2}", self.method.to_string(), blur),
                );
                blur < self.threshold
            })
            .collect();
        Ok(output)
    }
}