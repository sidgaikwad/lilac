use async_trait::async_trait;
use common::model::step_definition::{StepDefinition, StepType};
use uuid::uuid;

use crate::pipeline::{ImagePipe, PipeDefinition, PipeError, PipeImageData};

pub struct NoOpStep;

impl PipeDefinition for NoOpStep {
    fn step_definition() -> StepDefinition {
        StepDefinition {
            step_definition_id: uuid!("6c5d8079-63e9-4396-9369-2a9dda0f3fd9").into(),
            step_type: StepType::NoOp,
            parameter_definitions: vec![],
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
