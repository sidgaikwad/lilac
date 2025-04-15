use common::model::step_definition::StepDefinition;
use pipe_core::PipeDefinition;
use pipes::{blur::BlurDetectorPipe, noop::NoOpStep, resolution::ResolutionStandardizerPipe};

pub mod datasource;
pub mod destination;
pub mod pipe_core;
pub mod pipeline_definition;
pub mod pipes;
pub mod runner;
pub mod utils;

pub enum Pipe {
    BlurDetector(BlurDetectorPipe),
    ResolutionStandardizer(ResolutionStandardizerPipe),
    NoOp(NoOpStep),
}

pub fn get_steps_to_register() -> Vec<StepDefinition> {
    vec![
        BlurDetectorPipe::step_definition(),
        ResolutionStandardizerPipe::step_definition(),
        NoOpStep::step_definition(),
    ]
}
