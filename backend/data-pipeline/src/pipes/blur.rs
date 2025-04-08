//! Pipe implementation for detecting blurry images using configurable methods.
//! Methods: LaplacianVariance, EdgeIntensity (Sobel Mean), PixelVariance, EdgeCount (Sobel).

use crate::pipeline::{ImagePipe, PipeImageData, PipeResult, PipeConfig};
use crate::utils::log_pipe_event;
use async_trait::async_trait;
use image::{GrayImage, Luma};
use imageproc::{filter, gradients};
use serde_json::Value;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum BlurDetectionMethod {
    LaplacianVariance, // Laplcain Variance
    EdgeIntensity, // Mean of Sobel gradient magnitudes
    PixelVariance, // Variance of grayscale pixel intensities
    EdgeCount,     // Count of Sobel gradient magnitudes above a threshold
}

#[derive(Debug)]
struct BlurParams {
    method: BlurDetectionMethod,
    threshold: f64,
    // Only used for EdgeCount method
    edge_magnitude_threshold: Option<u16>,
}

/// A pipe that detects blurry images based on a configured method and threshold.
#[derive(Debug)]
pub struct BlurDetectorPipe;

impl BlurDetectorPipe {
    /// Extracts parameters: detection method and the relevant thresholds.
    fn get_params(&self, config: &PipeConfig) -> Result<BlurParams, String> {
        // Determine detection method (default to LaplacianVariance)
        let method_str = config
            .parameters
            .get("detection_method")
            .and_then(Value::as_str)
            .unwrap_or("laplacian_variance"); // Default method

        let method = match method_str.to_lowercase().as_str() {
            "laplacian_variance" => BlurDetectionMethod::LaplacianVariance,
            "edge_intensity" => BlurDetectionMethod::EdgeIntensity,
            "pixel_variance" => BlurDetectionMethod::PixelVariance,
            "edge_count" => BlurDetectionMethod::EdgeCount,
            _ => return Err(format!("Unknown detection_method: {}", method_str)),
        };

        // Get the primary threshold specific to the chosen method
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
            .ok_or_else(|| format!("Missing or invalid '{}' parameter (must be a number)", threshold_key))?;

        if threshold < 0.0 {
            return Err(format!("{} cannot be negative", threshold_key));
        }

        // Get the edge magnitude threshold *only* if using EdgeCount method
        let mut edge_magnitude_threshold: Option<u16> = None;
        if method == BlurDetectionMethod::EdgeCount {
            let mag_threshold = config
                .parameters
                .get("edge_magnitude_threshold")
                .and_then(Value::as_u64) // Expecting u16, parse as u64 first
                .map(|v| v as u16) // Convert to u16
                .ok_or_else(|| "Missing or invalid 'edge_magnitude_threshold' parameter for EdgeCount method (must be a positive integer)".to_string())?;
            edge_magnitude_threshold = Some(mag_threshold);
        }

        Ok(BlurParams {
            method,
            threshold,
            edge_magnitude_threshold,
        })
    }

    /// Calculates the variance of pixel values in image (Laplacian output).
    fn calculate_laplacian_variance(laplacian_img: &image::ImageBuffer<Luma<i16>, Vec<i16>>) -> f64 {
        let count = laplacian_img.pixels().len() as f64;
        if count == 0.0 { return 0.0; }
        let mut sum = 0.0;
        let mut sum_sq = 0.0;
        for pixel_val in laplacian_img.pixels().map(|p| p[0] as f64) {
            sum += pixel_val;
            sum_sq += pixel_val * pixel_val;
        }
        let mean = sum / count;
        (sum_sq / count) - (mean * mean) // Variance
    }

    /// Calculates the mean pixel value in image (Sobel gradient magnitude output).
    fn calculate_mean_gradient_intensity(gradient_img: &image::ImageBuffer<Luma<u16>, Vec<u16>>) -> f64 {
        let count = gradient_img.pixels().len() as f64;
        if count == 0.0 { return 0.0; }
        let mut sum = 0.0;
        for pixel_val in gradient_img.pixels().map(|p| p[0] as f64) {
            sum += pixel_val;
        }
        sum / count // Mean
    }

     /// Calculates the variance of pixel values in a grayscale image.
    fn calculate_pixel_variance(gray_img: &GrayImage) -> f64 {
        let count = gray_img.pixels().len() as f64;
        if count == 0.0 { return 0.0; }
        let mut sum = 0.0;
        let mut sum_sq = 0.0;
        for pixel_val in gray_img.pixels().map(|p| p[0] as f64) {
            sum += pixel_val;
            sum_sq += pixel_val * pixel_val;
        }
        let mean = sum / count;
        (sum_sq / count) - (mean * mean) // Variance
    }

    /// Counts pixels in a gradient image with magnitude above a threshold.
    fn calculate_edge_count(gradient_img: &image::ImageBuffer<Luma<u16>, Vec<u16>>, magnitude_threshold: u16) -> f64 {
        let mut edge_pixel_count = 0;
        for pixel_val in gradient_img.pixels().map(|p| p[0]) {
            if pixel_val >= magnitude_threshold {
                edge_pixel_count += 1;
            }
        }
        edge_pixel_count as f64 // Return count as f64 to match other metrics
    }
}

#[async_trait]
impl ImagePipe for BlurDetectorPipe {
    fn name(&self) -> &'static str {
        "BlurDetector"
    }

    async fn process(
        &self,
        mut data: PipeImageData,
        config: &PipeConfig,
    ) -> PipeResult {
        let pipe_name = self.name();
        let image_id = &data.id;

        log_pipe_event(pipe_name, image_id, "INFO", "Processing image for blur detection.");

        // 1. Get parameters
        let params = match self.get_params(config) {
            Ok(p) => p,
            Err(e) => {
                log_pipe_event(pipe_name, image_id, "ERROR", &format!("Invalid configuration: {}", e));
                return PipeResult::Error { message: format!("Invalid configuration: {}", e) };
            }
        };
        log_pipe_event(pipe_name, image_id, "DEBUG", &format!("Using params: {:?}", params));

        // 2. Convert to grayscale (needed for all methods here)
        // Use Luma8 as a common format for input to filters/calculations
        let gray_image = data.image.to_luma8();

        // 3. Calculate blur metric based on chosen method
        let (metric_name, metric_value) = match params.method {
            BlurDetectionMethod::LaplacianVariance => {
                let laplacian_image = filter::laplacian_filter(&gray_image);
                let variance = Self::calculate_laplacian_variance(&laplacian_image);
                ("laplacian_variance", variance)
            }
            BlurDetectionMethod::EdgeIntensity => {
                let gradients = gradients::sobel_gradients(&gray_image);
                let mean_intensity = Self::calculate_mean_gradient_intensity(&gradients);
                ("edge_intensity_mean", mean_intensity)
            }
            BlurDetectionMethod::PixelVariance => {
                let variance = Self::calculate_pixel_variance(&gray_image);
                ("pixel_variance", variance)
            }
            BlurDetectionMethod::EdgeCount => {
                let mag_threshold = params.edge_magnitude_threshold.unwrap();
                let gradients = gradients::sobel_gradients(&gray_image);
                let count = Self::calculate_edge_count(&gradients, mag_threshold);
                ("edge_count", count)
            }
        };

        log_pipe_event(pipe_name, image_id, "INFO", &format!("Calculated {}: {:.2}", metric_name, metric_value));

        // 4. Add metric to metadata
        data.metadata.insert(metric_name.to_string(), metric_value.into());
        // Also store method used and threshold for clarity
        data.metadata.insert("blur_detection_method".to_string(), format!("{:?}", params.method).to_lowercase().into());
        data.metadata.insert(format!("{}_threshold", metric_name), params.threshold.into());
         if let Some(mag_thresh) = params.edge_magnitude_threshold {
             data.metadata.insert("edge_magnitude_threshold".to_string(), mag_thresh.into());
         }


        // 5. Compare metric to threshold and decide outcome
        if metric_value < params.threshold {
            let reason = format!("{} {:.2} is below threshold {}", metric_name, metric_value, params.threshold);
            log_pipe_event(pipe_name, image_id, "INFO", &format!("Discarding image: {}", reason));
            PipeResult::Discarded { reason }
        } else {
            log_pipe_event(pipe_name, image_id, "INFO", &format!("Image passed {} check.", metric_name));
            PipeResult::Unchanged(data) // Image data not modified
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*; // Import items from the parent module (pipe implementation)
    use crate::pipeline::{PipeImageData, PipeConfig, PipeResult, ImageMetadata};
    use image::{DynamicImage, GrayImage, ImageFormat, Luma}; // Added GrayImage, RgbImage
    use imageproc::filter::gaussian_blur_f32; // For creating blurry test images
    use serde_json::json;
    use std::collections::HashMap;

    // --- Test Image Generation ---

    // Creates a simple high-contrast "sharp" grayscale image (e.g., lines)
    fn create_sharp_image(width: u32, height: u32) -> GrayImage {
        let mut img = GrayImage::new(width, height);
        for y in 0..height {
            for x in 0..width {
                // Simple pattern: alternating dark/light columns
                let intensity = if (x / 4) % 2 == 0 { 0 } else { 255 };
                img.put_pixel(x, y, Luma([intensity]));
            }
        }
        img
    }

    // Creates a blurry version of an image using Gaussian blur
    fn create_blurry_image(sharp_img: &GrayImage, sigma: f32) -> GrayImage {
        gaussian_blur_f32(sharp_img, sigma)
    }

    // Helper to create PipeImageData from a GrayImage
    fn create_test_data_from_gray(id: &str, gray_img: GrayImage) -> PipeImageData {
        let metadata: ImageMetadata = HashMap::new();
        PipeImageData {
            id: id.to_string(),
            image: DynamicImage::ImageLuma8(gray_img), // Store as DynamicImage
            metadata,
            original_format: ImageFormat::Png,
        }
    }

    // Helper to create PipeConfig
    fn create_test_config(params: HashMap<String, serde_json::Value>) -> PipeConfig {
        PipeConfig { parameters: params }
    }

    // --- Test Cases ---

    const TEST_W: u32 = 64;
    const TEST_H: u32 = 64;
    const BLUR_SIGMA: f32 = 3.0; // Amount of blur for test images

    // --- Laplacian Variance Tests ---

    #[tokio::test]
    async fn test_laplacian_sharp_passes() {
        let pipe = BlurDetectorPipe;
        let sharp_img = create_sharp_image(TEST_W, TEST_H);
        let data = create_test_data_from_gray("sharp_lap", sharp_img);
        let config = create_test_config(HashMap::from([
            ("detection_method".to_string(), json!("laplacian_variance")),
            ("laplacian_threshold".to_string(), json!(50.0)), // Relatively low threshold
        ]));

        let result = pipe.process(data, &config).await;
        assert!(matches!(result, PipeResult::Unchanged(_)), "Sharp image should pass low threshold");
        if let PipeResult::Unchanged(d) = result {
             assert!(d.metadata.contains_key("laplacian_variance"));
             // Check variance is likely > threshold (exact value depends on pattern/impl)
             assert!(d.metadata["laplacian_variance"].as_f64().unwrap_or(0.0) > 50.0);
        }
    }

    #[tokio::test]
    async fn test_laplacian_blurry_discarded() {
        let pipe = BlurDetectorPipe;
        let sharp_img = create_sharp_image(TEST_W, TEST_H);
        let blurry_img = create_blurry_image(&sharp_img, BLUR_SIGMA);
        let data = create_test_data_from_gray("blurry_lap", blurry_img);
        let config = create_test_config(HashMap::from([
            ("detection_method".to_string(), json!("laplacian_variance")),
            ("laplacian_threshold".to_string(), json!(50.0)), // Same threshold
        ]));

        let result = pipe.process(data, &config).await;
        assert!(matches!(result, PipeResult::Discarded { .. }), "Blurry image should be discarded by higher threshold");
    }

    // --- Edge Intensity (Mean) Tests ---

    #[tokio::test]
    async fn test_edge_intensity_sharp_passes() {
        let pipe = BlurDetectorPipe;
        let sharp_img = create_sharp_image(TEST_W, TEST_H);
        let data = create_test_data_from_gray("sharp_edge_mean", sharp_img);
        let config = create_test_config(HashMap::from([
            ("detection_method".to_string(), json!("edge_intensity")),
            ("edge_intensity_threshold".to_string(), json!(100.0)), // Low threshold for mean gradient
        ]));

        let result = pipe.process(data, &config).await;
        assert!(matches!(result, PipeResult::Unchanged(_)), "Sharp image should pass low edge intensity threshold");
         if let PipeResult::Unchanged(d) = result {
             assert!(d.metadata.contains_key("edge_intensity_mean"));
             assert!(d.metadata["edge_intensity_mean"].as_f64().unwrap_or(0.0) > 100.0);
        }
    }

    #[tokio::test]
    async fn test_edge_intensity_blurry_discarded() {
        let pipe = BlurDetectorPipe;
        let sharp_img = create_sharp_image(TEST_W, TEST_H);
        let blurry_img = create_blurry_image(&sharp_img, BLUR_SIGMA);
        let data = create_test_data_from_gray("blurry_edge_mean", blurry_img);
         let config = create_test_config(HashMap::from([
            ("detection_method".to_string(), json!("edge_intensity")),
            ("edge_intensity_threshold".to_string(), json!(100.0)), // Same threshold
        ]));

        let result = pipe.process(data, &config).await;
        assert!(matches!(result, PipeResult::Discarded { .. }), "Blurry image should be discarded by edge intensity threshold");
    }


    // --- Pixel Variance Tests ---

    #[tokio::test]
    async fn test_pixel_variance_sharp_passes() {
        let pipe = BlurDetectorPipe;
        let sharp_img = create_sharp_image(TEST_W, TEST_H); // High contrast image
        let data = create_test_data_from_gray("sharp_pix_var", sharp_img);
        let config = create_test_config(HashMap::from([
            ("detection_method".to_string(), json!("pixel_variance")),
            ("pixel_variance_threshold".to_string(), json!(1000.0)), // Low threshold
        ]));

        let result = pipe.process(data, &config).await;
        assert!(matches!(result, PipeResult::Unchanged(_)), "Sharp image should pass low pixel variance threshold");
        if let PipeResult::Unchanged(d) = result {
             assert!(d.metadata.contains_key("pixel_variance"));
             assert!(d.metadata["pixel_variance"].as_f64().unwrap_or(0.0) > 1000.0);
        }
    }

     #[tokio::test]
    async fn test_pixel_variance_blurry_discarded() {
        let pipe = BlurDetectorPipe;
        let sharp_img = create_sharp_image(TEST_W, TEST_H);
        let blurry_img = create_blurry_image(&sharp_img, BLUR_SIGMA); // Blur reduces variance
        let data = create_test_data_from_gray("blurry_pix_var", blurry_img);
        let config = create_test_config(HashMap::from([
            ("detection_method".to_string(), json!("pixel_variance")),
            ("pixel_variance_threshold".to_string(), json!(1000.0)), // Same threshold
        ]));

        let result = pipe.process(data, &config).await;
        assert!(matches!(result, PipeResult::Discarded { .. }), "Blurry image should be discarded by pixel variance threshold");
    }

    // --- Edge Count Tests ---

     #[tokio::test]
    async fn test_edge_count_sharp_passes() {
        let pipe = BlurDetectorPipe;
        let sharp_img = create_sharp_image(TEST_W, TEST_H);
        let data = create_test_data_from_gray("sharp_edge_count", sharp_img);
        let config = create_test_config(HashMap::from([
            ("detection_method".to_string(), json!("edge_count")),
            ("edge_count_threshold".to_string(), json!(100.0)), // Expect at least 100 edge pixels
            ("edge_magnitude_threshold".to_string(), json!(200)), // Define what counts as an edge (gradient magnitude)
        ]));

        let result = pipe.process(data, &config).await;
        assert!(matches!(result, PipeResult::Unchanged(_)), "Sharp image should pass low edge count threshold");
         if let PipeResult::Unchanged(d) = result {
             assert!(d.metadata.contains_key("edge_count"));
             assert!(d.metadata["edge_count"].as_f64().unwrap_or(0.0) > 100.0);
        }
    }

     #[tokio::test]
    async fn test_edge_count_blurry_discarded() {
        let pipe = BlurDetectorPipe;
        let sharp_img = create_sharp_image(TEST_W, TEST_H);
        let blurry_img = create_blurry_image(&sharp_img, BLUR_SIGMA);
        let data = create_test_data_from_gray("blurry_edge_count", blurry_img);
        let config = create_test_config(HashMap::from([
            ("detection_method".to_string(), json!("edge_count")),
            ("edge_count_threshold".to_string(), json!(100.0)), // Same count threshold
            ("edge_magnitude_threshold".to_string(), json!(5000)), // Same magnitude threshold
        ]));

        let result = pipe.process(data, &config).await;
        assert!(matches!(result, PipeResult::Discarded { .. }), "Blurry image should be discarded by edge count threshold");
    }

     #[tokio::test]
    async fn test_edge_count_missing_magnitude_threshold() {
        let pipe = BlurDetectorPipe;
        let data = create_test_data_from_gray("edge_count_err", create_sharp_image(TEST_W, TEST_H));
        let config = create_test_config(HashMap::from([
            ("detection_method".to_string(), json!("edge_count")),
            ("edge_count_threshold".to_string(), json!(100.0)),
            // Missing "edge_magnitude_threshold"
        ]));

        let result = pipe.process(data, &config).await;
        assert!(matches!(result, PipeResult::Error { .. }), "Expected Error if edge_magnitude_threshold is missing for EdgeCount");
    }

    // --- General Error Tests ---

    #[tokio::test]
    async fn test_missing_primary_threshold() {
        let pipe = BlurDetectorPipe;
        let data = create_test_data_from_gray("missing_thresh", create_sharp_image(TEST_W, TEST_H));
        let config = create_test_config(HashMap::from([
             // Using default method (laplacian_variance) but missing its threshold
            ("detection_method".to_string(), json!("laplacian_variance")),
        ]));

        let result = pipe.process(data, &config).await;
        assert!(matches!(result, PipeResult::Error { .. }), "Expected Error if primary threshold is missing");
    }

    #[tokio::test]
    async fn test_unknown_method() {
        let pipe = BlurDetectorPipe;
        let data = create_test_data_from_gray("unknown_method", create_sharp_image(TEST_W, TEST_H));
        let config = create_test_config(HashMap::from([
            ("detection_method".to_string(), json!("some_unknown_method")),
            ("laplacian_threshold".to_string(), json!(100.0)), // Threshold doesn't matter here
        ]));

        let result = pipe.process(data, &config).await;
        assert!(matches!(result, PipeResult::Error { .. }), "Expected Error for unknown detection method");
    }
}
