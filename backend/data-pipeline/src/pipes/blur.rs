use crate::pipe_core::{ImagePipe, PipeDefinition, PipeError, PipeImageData};
use crate::utils::log_pipe_event;
use async_trait::async_trait;
use common::model::step_definition::{StepCategory, StepDefinition, StepType};
use image::{DynamicImage, GrayImage, Luma};
use imageproc::{filter, gradients};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_json::json;
use strum::{Display, EnumString};
use uuid::uuid;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Display, EnumString)]
pub enum BlurDetectionMethod {
    LaplacianVariance,
    EdgeIntensity,
    PixelVariance,
    EdgeCount { edge_magnitude_threshold: u16 },
}

#[derive(Debug, Clone)]
pub struct BlurPipe {
    method: BlurDetectionMethod,
    threshold: f64,
}

impl BlurPipe {
    pub fn new(method: BlurDetectionMethod, threshold: f64) -> Result<Self, String> {
        if threshold < 0.0 {
            return Err("Threshold cannot be negative.".to_string());
        }
        Ok(Self { method, threshold })
    }

    fn calculate_blur_metric(&self, img: &DynamicImage) -> f64 {
        let gray_image = img.to_luma8();
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

    fn calculate_laplacian_variance(
        &self,
        laplacian_img: &image::ImageBuffer<Luma<i16>, Vec<i16>>,
    ) -> f64 {
        let count = laplacian_img.pixels().len() as f64;
        if count == 0.0 { return 0.0; }
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
        if count == 0.0 { return 0.0; }
        let sum: f64 = gradient_img.pixels().map(|p| p[0] as f64).sum();
        sum / count
    }

    fn calculate_pixel_variance(&self, gray_img: &GrayImage) -> f64 {
        let count = gray_img.pixels().len() as f64;
        if count == 0.0 { return 0.0; }
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
        gradient_img.pixels().filter(|p| p[0] >= magnitude_threshold).count() as f64
    }
}

impl PipeDefinition for BlurPipe {
    fn step_definition() -> StepDefinition {
        StepDefinition {
            id: uuid!("039fddf8-72c1-4598-875c-36f40d4fcf84").into(),
            name: "Blur Detector".into(),
            description: Some("Filters out images that match a certain blur threshold.".into()),
            category: StepCategory::ImageProcessing,
            step_type: StepType::BlurDetector,
            inputs: vec!["input".into()],
            outputs: vec!["output".into()],
            schema: json!({
                "type": "object",
                "properties": {
                    "threshold": { "type": "number", "minimum": 0.0 },
                    "method": { "enum": ["LaplacianVariance", "EdgeIntensity", "PixelVariance", "EdgeCount"] }
                },
                "allOf": [
                    {
                        "if": { "properties": { "method": { "const": "EdgeCount" } } },
                        "then": {
                            "properties": { "edge_magnitude_threshold": { "type": "integer", "minimum": 0 } },
                            "required": ["edge_magnitude_threshold"]
                        }
                    }
                ],
                "required": ["threshold", "method"]
            }),
        }
    }
}

#[async_trait]
impl ImagePipe for BlurPipe {
    fn name(&self) -> &'static str {
        "BlurDetector"
    }

    async fn run_stage(
        &self,
        image_batch: Vec<PipeImageData>,
    ) -> Result<Vec<PipeImageData>, PipeError> {
        let pipe_name = self.name();
        let method = self.method;
        let threshold = self.threshold;
        let input_len = image_batch.len();

        log_pipe_event(
            pipe_name,
            "STAGE_INIT",
            "DEBUG",
            &format!("Stage configured with params: {:?} threshold: {}", method, threshold),
        );

        let output_batch: Vec<PipeImageData> = image_batch
            .into_par_iter()
            .filter(|img_data| {
                let blur_metric = self.calculate_blur_metric(&img_data.image);
                let keep = blur_metric >= threshold;
                // Optional logging for discarded images
                if !keep {
                     log_pipe_event(
                        pipe_name,
                        &img_data.id,
                        "INFO", 
                        &format!("Discarding image (metric {} < threshold {})", blur_metric, threshold),
                    );
                }
                keep
            })
            .collect();

        log_pipe_event(
            pipe_name,
            "STAGE_COMPLETE",
            "DEBUG",
            &format!("Stage finished. Input: {}, Output: {}", input_len, output_batch.len()),
        );

        Ok(output_batch)
    }
}
