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
pub enum RotationAngle {
    Rotate90,
    Rotate180,
    Rotate270,
}

#[derive(Debug, Clone)]
pub struct RotatePipe {
    angle: RotationAngle,
}

impl RotatePipe {
    pub fn new(angle: RotationAngle) -> Self {
        Self { angle }
    }
}

impl PipeDefinition for RotatePipe {
    fn step_definition() -> StepDefinition {
        StepDefinition {
            id: uuid!("b8c3d2e1-f0a1-4b4c-9d8e-7f6a5e4d3c2b").into(),
            step_type: StepType::Rotate,
            name: "Rotate Image".into(),
            description: Some("Rotates all images by the provided angle.".into()),
            category: StepCategory::ImageProcessing,
            inputs: vec!["input".into()],
            outputs: vec!["output".into()],
            schema: json!({
                "type": "object",
                "properties": {
                    "angle": {
                        "type": "string",
                        "enum": ["rotate90", "rotate180", "rotate270"],
                    }
                },
                "required": ["angle"],
            }),
        }
    }
}

#[async_trait]
impl ImagePipe for RotatePipe {
    fn name(&self) -> &'static str {
        "Rotate"
    }

    async fn run_stage(
        &self,
        image_batch: Vec<PipeImageData>,
    ) -> Result<Vec<PipeImageData>, PipeError> {
        let angle = self.angle;

        let output_batch: Vec<PipeImageData> = image_batch
            .into_par_iter()
            .map(move |mut img_data| {
                img_data.image = match angle {
                    RotationAngle::Rotate90 => {
                        DynamicImage::ImageRgba8(imageops::rotate90(&img_data.image))
                    }
                    RotationAngle::Rotate180 => {
                        DynamicImage::ImageRgba8(imageops::rotate180(&img_data.image))
                    }
                    RotationAngle::Rotate270 => {
                        DynamicImage::ImageRgba8(imageops::rotate270(&img_data.image))
                    }
                };
                img_data
            })
            .collect();

        Ok(output_batch)
    }
}