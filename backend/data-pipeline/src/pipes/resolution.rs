    //! Pipe implementation for standardizing image resolution.

    use crate::pipeline::{ImagePipe, PipeImageData, PipeResult, PipeConfig};
    use crate::utils::log_pipe_event; // Use the logging facade
    use async_trait::async_trait;
    use image::imageops::{self, FilterType};
    use image::GenericImageView;
    use serde_json::Value;

    /// A pipe that resizes images to a target width and height.
    #[derive(Debug)]
    pub struct ResolutionStandardizerPipe;

    impl ResolutionStandardizerPipe {
        /// Extracts resizing parameters from the configuration.
        /// Provides defaults if parameters are missing or invalid.
        fn get_params(&self, config: &PipeConfig) -> Result<(u32, u32, FilterType), String> {
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

            // Default to Lanczos3 if filter_type is missing or invalid
            let filter_type_str = config
                .parameters
                .get("filter_type")
                .and_then(Value::as_str)
                .unwrap_or("Lanczos3"); // Default filter

            let filter_type = match filter_type_str.to_lowercase().as_str() {
                "nearest" => FilterType::Nearest,
                "triangle" => FilterType::Triangle,
                "catmullrom" => FilterType::CatmullRom,
                "gaussian" => FilterType::Gaussian,
                "lanczos3" => FilterType::Lanczos3,
                _ => {
                    // Log a warning about invalid filter type and use default
                    // Note: Can't easily log here without image_id, maybe return warning string?
                    // For simplicity now, just default. Proper logging would happen in process().
                    FilterType::Lanczos3
                }
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

        async fn process(
            &self,
            mut data: PipeImageData, // Make data mutable
            config: &PipeConfig,
        ) -> PipeResult {
            let pipe_name = self.name();
            let image_id = &data.id;

            log_pipe_event(pipe_name, image_id, "INFO", "Processing image for resolution standardization.");

            // 1. Get parameters
            let params = match self.get_params(config) {
                Ok(p) => p,
                Err(e) => {
                    log_pipe_event(pipe_name, image_id, "ERROR", &format!("Invalid configuration: {}", e));
                    // Return Error for configuration issues
                    return PipeResult::Error { message: format!("Invalid configuration: {}", e) };
                }
            };
            let (target_width, target_height, filter_type) = params;

            // Log the parameters being used
            log_pipe_event(pipe_name, image_id, "DEBUG", &format!("Target dimensions: {}x{}, Filter: {:?}", target_width, target_height, filter_type));


            // 2. Check if resizing is needed
            let (current_width, current_height) = data.image.dimensions();
            if current_width == target_width && current_height == target_height {
                log_pipe_event(pipe_name, image_id, "INFO", "Image already has target dimensions. Skipping resize.");
                return PipeResult::Unchanged(data);
            }

            // 3. Perform resizing
            log_pipe_event(pipe_name, image_id, "INFO", &format!("Resizing image from {}x{} to {}x{}", current_width, current_height, target_width, target_height));
            let resized_image = imageops::resize(
                &data.image,
                target_width,
                target_height,
                filter_type,
            );

            // 4. Update data and metadata
            data.image = resized_image.into(); // Convert RgbaImage back to DynamicImage
            // Optionally update metadata
            data.metadata.insert("resized_width".to_string(), target_width.into());
            data.metadata.insert("resized_height".to_string(), target_height.into());
            data.metadata.insert("original_width_before_resize".to_string(), current_width.into());
            data.metadata.insert("original_height_before_resize".to_string(), current_height.into());


            log_pipe_event(pipe_name, image_id, "INFO", "Image successfully resized.");

            // 5. Return modified data
            PipeResult::Modified(data)
        }
    }


    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::pipeline::{PipeImageData, PipeConfig, PipeResult, ImageMetadata};
        use image::{DynamicImage, RgbImage, ImageFormat};
        use serde_json::json;
        use std::collections::HashMap;
    
        // Helper function to create a simple PipeImageData for testing
        fn create_test_data(width: u32, height: u32) -> PipeImageData {
            let img: DynamicImage = DynamicImage::ImageRgb8(RgbImage::new(width, height));
            let metadata: ImageMetadata = HashMap::new();
            PipeImageData {
                id: format!("test_{}x{}", width, height),
                image: img,
                metadata,
                original_format: ImageFormat::Png,
            }
        }
    
        // Helper function to create a PipeConfig
        fn create_test_config(params: HashMap<String, serde_json::Value>) -> PipeConfig {
            PipeConfig { parameters: params }
        }
    
        #[tokio::test]
        async fn test_resize_down_valid() {
            let pipe = ResolutionStandardizerPipe;
            let data = create_test_data(200, 200); // Initial size 200x200
            let config = create_test_config(HashMap::from([
                ("target_width".to_string(), json!(100)),
                ("target_height".to_string(), json!(50)),
            ]));
    
            let result = pipe.process(data, &config).await;
    
            assert!(matches!(result, PipeResult::Modified(_)), "Expected Modified result");
            if let PipeResult::Modified(output_data) = result {
                assert_eq!(output_data.image.width(), 100, "Width should be 100");
                assert_eq!(output_data.image.height(), 50, "Height should be 50");
                assert_eq!(output_data.metadata.get("resized_width"), Some(&json!(100)));
                assert_eq!(output_data.metadata.get("resized_height"), Some(&json!(50)));
                assert_eq!(output_data.metadata.get("original_width_before_resize"), Some(&json!(200)));
            }
        }
    
        #[tokio::test]
        async fn test_resize_up_valid() {
            let pipe = ResolutionStandardizerPipe;
            let data = create_test_data(50, 50); // Initial size 50x50
            let config = create_test_config(HashMap::from([
                ("target_width".to_string(), json!(100)),
                ("target_height".to_string(), json!(150)),
            ]));
    
            let result = pipe.process(data, &config).await;
    
            assert!(matches!(result, PipeResult::Modified(_)), "Expected Modified result");
            if let PipeResult::Modified(output_data) = result {
                assert_eq!(output_data.image.width(), 100, "Width should be 100");
                assert_eq!(output_data.image.height(), 150, "Height should be 150");
            }
        }
    
        #[tokio::test]
        async fn test_already_correct_size() {
            let pipe = ResolutionStandardizerPipe;
            let data = create_test_data(100, 100); // Initial size 100x100
            let config = create_test_config(HashMap::from([
                ("target_width".to_string(), json!(100)),
                ("target_height".to_string(), json!(100)),
            ]));
    
            let result = pipe.process(data, &config).await;
    
            assert!(matches!(result, PipeResult::Unchanged(_)), "Expected Unchanged result");
             if let PipeResult::Unchanged(output_data) = result {
                assert_eq!(output_data.image.width(), 100); // Dimensions remain the same
                assert_eq!(output_data.image.height(), 100);
                assert!(output_data.metadata.get("resized_width").is_none(), "Metadata should not be added if unchanged");
            }
        }
    
        #[tokio::test]
        async fn test_missing_parameters() {
            let pipe = ResolutionStandardizerPipe;
            let data = create_test_data(100, 100);
            // Config missing target_height
            let config = create_test_config(HashMap::from([
                ("target_width".to_string(), json!(50)),
            ]));
    
            let result = pipe.process(data, &config).await;
    
            assert!(matches!(result, PipeResult::Error { .. }), "Expected Error result for missing params");
             if let PipeResult::Error { message } = result {
                assert!(message.contains("Missing or invalid 'target_height' parameter"));
            }
        }
    
         #[tokio::test]
        async fn test_invalid_parameters_zero_width() {
            let pipe = ResolutionStandardizerPipe;
            let data = create_test_data(100, 100);
            let config = create_test_config(HashMap::from([
                ("target_width".to_string(), json!(0)), // Invalid width
                ("target_height".to_string(), json!(50)),
            ]));
    
            let result = pipe.process(data, &config).await;
    
            assert!(matches!(result, PipeResult::Error { .. }), "Expected Error result for zero width");
             if let PipeResult::Error { message } = result {
                assert!(message.contains("Target width and height must be greater than 0"));
            }
        }
    
         #[tokio::test]
        async fn test_invalid_parameters_wrong_type() {
            let pipe = ResolutionStandardizerPipe;
            let data = create_test_data(100, 100);
            let config = create_test_config(HashMap::from([
                ("target_width".to_string(), json!("not a number")), // Invalid type
                ("target_height".to_string(), json!(50)),
            ]));
    
            let result = pipe.process(data, &config).await;
    
            assert!(matches!(result, PipeResult::Error { .. }), "Expected Error result for wrong type");
             if let PipeResult::Error { message } = result {
                assert!(message.contains("Missing or invalid 'target_width' parameter"));
            }
        }
    
         #[tokio::test]
        async fn test_different_filter_type() {
            let pipe = ResolutionStandardizerPipe;
            let data = create_test_data(200, 200);
            let config = create_test_config(HashMap::from([
                ("target_width".to_string(), json!(100)),
                ("target_height".to_string(), json!(100)),
                ("filter_type".to_string(), json!("Nearest")), // Use different filter
            ]));
    
            let result = pipe.process(data, &config).await;
    
            // Just check it runs and modifies, doesn't necessarily check pixel differences
            assert!(matches!(result, PipeResult::Modified(_)), "Expected Modified result with Nearest filter");
            if let PipeResult::Modified(output_data) = result {
                assert_eq!(output_data.image.width(), 100);
                assert_eq!(output_data.image.height(), 100);
            }
        }
    
         #[tokio::test]
        async fn test_default_filter_type() {
            let pipe = ResolutionStandardizerPipe;
            let data = create_test_data(200, 200);
             // Config missing filter_type, should default to Lanczos3
            let config = create_test_config(HashMap::from([
                ("target_width".to_string(), json!(100)),
                ("target_height".to_string(), json!(100)),
            ]));
    
            let result = pipe.process(data, &config).await;
    
            assert!(matches!(result, PipeResult::Modified(_)), "Expected Modified result with default filter");
             if let PipeResult::Modified(output_data) = result {
                assert_eq!(output_data.image.width(), 100);
                assert_eq!(output_data.image.height(), 100);
            }
        }
    }
    