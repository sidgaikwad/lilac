use async_trait::async_trait;
use common::model::step_definition::{StepCategory, StepDefinition, StepType};
use serde_json::json;
use uuid::uuid;

use crate::pipeline::{ImagePipe, PipeDefinition, PipeError, PipeImageData};

pub struct NoOpStep;

impl PipeDefinition for NoOpStep {
    fn step_definition() -> StepDefinition {
        StepDefinition {
            id: uuid!("6c5d8079-63e9-4396-9369-2a9dda0f3fd9").into(),
            name: "No Op".into(),
            description: Some("Does nothing.".into()),
            category: StepCategory::ImageProcessing,
            step_type: StepType::NoOp,
            inputs: vec!["input".into()],
            outputs: vec!["output".into()],
            schema: json!({}),
        }
    }
}

#[async_trait]
impl ImagePipe for NoOpStep {
    fn name(&self) -> &'static str {
        "NoOp"
    }

    async fn run_stage(
        &self,
        image_batch: Vec<PipeImageData>,
    ) -> Result<Vec<PipeImageData>, PipeError> {
        Ok(image_batch)
    }
}
