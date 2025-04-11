//! Pipe implementation for standardizing image resolution (Streaming/Channel Processing).
//! Implements the async run_stage method, using channels and spawn_blocking.

use crate::pipeline::{ImagePipe, PipeError, PipeImageData};
use crate::utils::log_pipe_event;
use async_trait::async_trait;
use image::imageops::{self, FilterType};
use image::DynamicImage;
use serde_json::json;

#[derive(Debug)]
pub struct ResolutionStandardizerPipe {
    target_width: u32,
    target_height: u32,
    filter_type: FilterType,
}

impl ResolutionStandardizerPipe {
    pub fn new(target_width: u32, target_height: u32, filter_type: FilterType) -> Self {
        Self {
            target_height,
            target_width,
            filter_type,
        }
    }

    fn resize_image(&self, img: DynamicImage) -> DynamicImage {
        let resized_image = imageops::resize(
            &img,
            self.target_width,
            self.target_height,
            self.filter_type,
        );
        DynamicImage::ImageRgba8(resized_image)
    }
}

#[async_trait]
impl ImagePipe for ResolutionStandardizerPipe {
    fn name(&self) -> &'static str {
        "ResolutionStandardizer"
    }

    fn param_definitions(&self) -> Vec<serde_json::Value> {
        // TODO: return a proper jsonschema instead
        vec![
            json!({
                "parameter_name": "target_width",
                "parameter_type": "number"
            }),
            json!({
                "parameter_name": "target_height",
                "parameter_type": "number"
            }),
            json!({
                "parameter_name": "filter_type",
                "parameter_type": "enum"
            }),
        ]
    }

    /// Runs the resolution standardization stage as an asynchronous task.
    /// Reads from input_rx, processes using spawn_blocking for CPU work,
    /// and sends results to output_tx.
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
                "Stage configured with Target: {}x{}, Filter: {:?}",
                self.target_width, self.target_height, self.filter_type
            ),
        );

        let output = image_batch
            .into_iter()
            .map(|mut img_data| {
                img_data.image = self.resize_image(img_data.image);
                img_data
            })
            .collect();

        log_pipe_event(
            pipe_name,
            "STAGE_EXIT",
            "INFO",
            "Input channel closed. Exiting stage task.",
        );

        Ok(output)
    } // End of run_stage
}
