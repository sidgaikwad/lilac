use common::model::step_definition::StepDefinition;
use pipeline::ImagePipe;
use pipes::{blur::BlurDetectorPipe, noop::NoOpStep, resolution::ResolutionStandardizerPipe};

pub mod pipeline;
pub mod pipes;
pub mod utils;

pub fn get_steps_to_register() -> Vec<StepDefinition> {
    return vec![
        BlurDetectorPipe::step_definition(),
        ResolutionStandardizerPipe::step_definition(),
        NoOpStep::step_definition(),
    ];
}
