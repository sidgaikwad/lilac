use common::model::step_definition::StepDefinition;
use pipe_core::PipeDefinition;

use pipes::{
    add_noise::AddNoisePipe, blur::BlurPipe, brightness::BrightnessPipe, contrast::ContrastPipe,
    flip::FlipPipe, grayscale::GrayscalePipe, noop::NoOpStep, resize::ResizePipe,
    rotate::RotatePipe,
};

pub mod datasource;
pub mod destination;
pub mod pipe_core;
pub mod pipeline_definition;
pub mod pipes;
pub mod runner;
pub mod utils;


pub enum Pipe {
    Blur(BlurPipe),
    Resize(ResizePipe),
    Rotate(RotatePipe),
    Flip(FlipPipe),
    Grayscale(GrayscalePipe),
    Brightness(BrightnessPipe),
    Contrast(ContrastPipe),
    AddNoise(AddNoisePipe),
    NoOp(NoOpStep),
}

/// Returns the StepDefinitions for all implemented pipes.
/// This can be used to register available steps with a control plane or UI.
pub fn get_steps_to_register() -> Vec<StepDefinition> {
    vec![
        BlurPipe::step_definition(),
        ResizePipe::step_definition(),
        RotatePipe::step_definition(),
        FlipPipe::step_definition(),
        GrayscalePipe::step_definition(),
        BrightnessPipe::step_definition(),
        ContrastPipe::step_definition(),
        AddNoisePipe::step_definition(),
        NoOpStep::step_definition(),
    ]
}
