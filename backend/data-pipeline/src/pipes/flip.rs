use crate::pipe_core::{ImagePipe, PipeDefinition, PipeError, PipeImageData};
use async_trait::async_trait;
use common::model::step_definition::StepCategory;
use common::model::step_definition::{StepDefinition, StepType};
use image::imageops;
use image::DynamicImage; // Needed for wrapping
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::{Deserialize, Serialize};
use serde_json::json;
use strum::{Display, EnumString};
use uuid::uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FlipDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone)]
pub struct FlipPipe {
    direction: FlipDirection,
}

impl FlipPipe {
    pub fn new(direction: FlipDirection) -> Self {
        Self { direction }
    }
}

impl PipeDefinition for FlipPipe {
    fn step_definition() -> StepDefinition {
        StepDefinition {
            id: uuid!("c9d4e3f2-a1b2-4c5d-ae9f-8a7b6c5e4d3c").into(),
            step_type: StepType::Flip,
            name: "Flip".into(),
            description: Some("Flips image vertically or horizontally.".into()),
            category: StepCategory::ImageProcessing,
            inputs: vec!["input".into()],
            outputs: vec!["output".into()],
            schema: json!({
                "type": "object",
                "properties": {
                    "direction": {
                        "type": "string",
                        "enum": ["horizontal", "vertical"],
                    }
                },
                "required": ["direction"],
            }),
        }
    }
}

#[async_trait]
impl ImagePipe for FlipPipe {
    fn name(&self) -> &'static str {
        "Flip"
    }

    async fn run_stage(
        &self,
        image_batch: Vec<PipeImageData>,
    ) -> Result<Vec<PipeImageData>, PipeError> {
        let direction = self.direction;

        let output_batch: Vec<PipeImageData> = image_batch
            .into_par_iter()
            .map(move |mut img_data| {
                // Call imageops and wrap the result
                img_data.image = match direction {
                    FlipDirection::Horizontal => {
                        DynamicImage::ImageRgba8(imageops::flip_horizontal(&img_data.image))
                    }
                    FlipDirection::Vertical => {
                        DynamicImage::ImageRgba8(imageops::flip_vertical(&img_data.image))
                    }
                };
                img_data
            })
            .collect();

        Ok(output_batch)
    }
}