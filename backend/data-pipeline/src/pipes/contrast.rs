use crate::pipe_core::{ImagePipe, PipeDefinition, PipeError, PipeImageData};
use async_trait::async_trait;
use common::model::step_definition::StepCategory;
use common::model::step_definition::{StepDefinition, StepType};
use image::imageops;
use image::DynamicImage;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde_json::json;
use uuid::uuid;

#[derive(Debug, Clone)]
pub struct ContrastPipe {
    contrast: f32,
}

impl ContrastPipe {
    pub fn new(contrast: f32) -> Self {
        Self { contrast }
    }
}

impl PipeDefinition for ContrastPipe {
    fn step_definition() -> StepDefinition {
        StepDefinition {
            id: uuid!("f2a7b6c5-d4e3-4f8a-b0c1-1d0e9f8a7b6c").into(),
            step_type: StepType::Contrast,
            name: "Contrast".into(),
            description: Some("Changes the contrast of the images.".into()),
            category: StepCategory::ImageProcessing,
            inputs: vec!["input".into()],
            outputs: vec!["output".into()],
            schema: json!({
                "type": "object",
                "properties": {
                    "contrast": {
                        "type": "number",
                    }
                },
                "required": ["contrast"],
            }),
        }
    }
}

#[async_trait]
impl ImagePipe for ContrastPipe {
    fn name(&self) -> &'static str {
        "Contrast"
    }

    async fn run_stage(
        &self,
        image_batch: Vec<PipeImageData>,
    ) -> Result<Vec<PipeImageData>, PipeError> {
        let contrast = self.contrast;

        let output_batch: Vec<PipeImageData> = image_batch
            .into_par_iter()
            .map(move |mut img_data| {
                let contrasted_buffer = imageops::contrast(&img_data.image, contrast);
                img_data.image = DynamicImage::ImageRgba8(contrasted_buffer);
                img_data
            })
            .collect();

        Ok(output_batch)
    }
}