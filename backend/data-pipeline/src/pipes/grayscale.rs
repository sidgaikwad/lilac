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
pub struct GrayscalePipe {}

impl GrayscalePipe {
    pub fn new() -> Self {
        Self {}
    }
}

impl PipeDefinition for GrayscalePipe {
    fn step_definition() -> StepDefinition {
        StepDefinition {
            id: uuid!("d0e5f4a3-b2c1-4d6e-bf0a-9b8c7d6e5f4d").into(),
            step_type: StepType::Grayscale,
            name: "Grayscale".into(),
            description: Some("Converts all images to grayscale.".into()),
            category: StepCategory::ImageProcessing,
            inputs: vec!["input".into()],
            outputs: vec!["output".into()],
            schema: json!({
                "type": "object",
                "properties": {},
            }),
        }
    }
}

#[async_trait]
impl ImagePipe for GrayscalePipe {
    fn name(&self) -> &'static str {
        "Grayscale"
    }

    async fn run_stage(
        &self,
        image_batch: Vec<PipeImageData>,
    ) -> Result<Vec<PipeImageData>, PipeError> {
        let output_batch: Vec<PipeImageData> = image_batch
            .into_par_iter()
            .map(|mut img_data| {
                img_data.image = DynamicImage::ImageLuma8(imageops::grayscale(&img_data.image));
                img_data
            })
            .collect();

        Ok(output_batch)
    }
}