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
    