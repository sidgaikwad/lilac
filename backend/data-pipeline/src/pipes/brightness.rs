use crate::pipe_core::{ImagePipe, PipeDefinition, PipeError, PipeImageData};
use async_trait::async_trait;
use common::model::step_definition::{StepDefinition, StepType};
use image::imageops;
use image::DynamicImage;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_json::json;
use uuid::uuid;

#[derive(Debug, Clone)]
pub struct BrightnessPipe {
    value: i32,
}

impl BrightnessPipe {
    pub fn new(value: i32) -> Self {
        Self { value }
    }
}

impl PipeDefinition for BrightnessPipe {
    fn step_definition() -> StepDefinition {
        StepDefinition {
            step_definition_id: uuid!("e1f6a5b4-c3d2-4e7f-af1b-0c9d8e7f6a5b").into(),
            step_type: StepType::Brightness,
            schema: json!({
                "type": "object",
                "properties": {
                    "value": {
                        "type": "integer",
                    }
                },
                "required": ["value"],
            }),
        }
    }
}

#[async_trait]
impl ImagePipe for BrightnessPipe {
    fn name(&self) -> &'static str {
        "Brightness"
    }

    async fn run_stage(
        &self,
        image_batch: Vec<PipeImageData>,
    ) -> Result<Vec<PipeImageData>, PipeError> {
        let value = self.value;

        let output_batch: Vec<PipeImageData> = image_batch
            .into_par_iter()
            .map(move |mut img_data| {
                let brightened_buffer = imageops::brighten(&img_data.image, value);
                img_data.image = DynamicImage::ImageRgba8(brightened_buffer);
                img_data
            })
            .collect();

        Ok(output_batch)
    }
}