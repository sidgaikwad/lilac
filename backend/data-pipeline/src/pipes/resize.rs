use crate::pipe_core::{ImagePipe, PipeDefinition, PipeError, PipeImageData};
use async_trait::async_trait;
use common::model::step_definition::{StepDefinition, StepType};
use image::imageops::{self, FilterType};
use image::DynamicImage;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_json::json;
use uuid::uuid;

#[derive(Debug, Clone)]
pub struct ResizePipe {
    target_width: u32,
    target_height: u32,
    filter_type: FilterType,
}

impl ResizePipe {
    pub fn new(target_width: u32, target_height: u32, filter_type: FilterType) -> Result<Self, String> {
        if target_width == 0 || target_height == 0 {
            Err("Target width and height must be positive.".to_string())
        } else {
            Ok(Self {
                target_height,
                target_width,
                filter_type,
            })
        }
    }
}

impl PipeDefinition for ResizePipe {
    fn step_definition() -> StepDefinition {
        StepDefinition {
            step_definition_id: uuid!("9a3601dd-d335-4cf9-99e0-01e928a3eec4").into(),
            step_type: StepType::ResolutionStandardizer,
            schema: json!({
                "type": "object",
                "properties": {
                    "target_height": { "type": "integer", "minimum": 1 },
                    "target_width": { "type": "integer", "minimum": 1 },
                    "filter_type": {
                        "type": "string",
                        "enum": ["Nearest", "Triangle", "CatmullRom", "Gaussian", "Lanczos3"],
                    }
                },
                "required": ["target_height", "target_width", "filter_type"],
            }),
        }
    }
}

#[async_trait]
impl ImagePipe for ResizePipe {
    fn name(&self) -> &'static str {
        "ResolutionStandardizer"
    }

    async fn run_stage(
        &self,
        image_batch: Vec<PipeImageData>,
    ) -> Result<Vec<PipeImageData>, PipeError> {
        let target_width = self.target_width;
        let target_height = self.target_height;
        let filter_type = self.filter_type;

        let output_batch: Vec<PipeImageData> = image_batch
            .into_par_iter()
            .map(move |mut img_data| {
                let resized_buffer = imageops::resize(
                    &img_data.image,
                    target_width,
                    target_height,
                    filter_type,
                );
                img_data.image = DynamicImage::ImageRgba8(resized_buffer);
                img_data
            })
            .collect();

        Ok(output_batch)
    }
}
