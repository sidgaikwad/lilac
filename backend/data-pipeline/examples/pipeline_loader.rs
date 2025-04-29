//! examples/pipeline_loader.rs
//! Demonstrates defining a pipeline configuration, building the runnable
//! pipeline instance (instantiating pipes via factory logic), and executing
//! it using the runner module.

use data_pipeline::pipe_core::{ImagePipe, PipelineStageConfig};
use data_pipeline::pipeline_definition::{DataDestination, DataSource, Pipeline};
use data_pipeline::pipes::{
    add_noise::{AddNoisePipe, NoiseType},
    blur::{BlurDetectionMethod, BlurPipe},
    brightness::BrightnessPipe,
    contrast::ContrastPipe,
    flip::{FlipDirection, FlipPipe},
    grayscale::GrayscalePipe,
    noop::NoOpStep, // Keep NoOp for completeness
    resize::ResizePipe,
    rotate::{RotatePipe, RotationAngle},
};
use data_pipeline::runner;
use image::imageops::FilterType;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr; // Required for parsing enums from string
use std::time::Instant;

const INPUT_DIR: &str = "test_images";
const OUTPUT_DIR: &str = "output"; // Ensure this directory exists or is created

// Helper function to extract and parse parameters
// (Using a simplified version here for brevity, assuming correct types in JSON for example)
fn get_param_value<'a>(
    params: &'a HashMap<String, Value>,
    key: &str,
    pipe_id: &str,
) -> Result<&'a Value, String> {
    params
        .get(key)
        .ok_or_else(|| format!("Missing parameter '{}' for {}", key, pipe_id))
}

// Helper specifically for parsing enums via FromStr
fn get_enum_param<T>(
    params: &HashMap<String, Value>,
    key: &str,
    pipe_id: &str,
) -> Result<T, String>
where
    T: FromStr,
    T::Err: std::fmt::Display,
{
    get_param_value(params, key, pipe_id)?
        .as_str()
        .ok_or_else(|| format!("Parameter '{}' must be a string for {}", key, pipe_id))
        .and_then(|s| T::from_str(s).map_err(|e| format!("Invalid value '{}' for parameter '{}' in {}: {}", s, key, pipe_id, e)))
}


/// Builds the pipeline stages by instantiating pipes based on configuration.
/// This acts as a simple factory.
fn build_pipeline_stages(
    stage_configs: &[PipelineStageConfig],
) -> Result<Vec<Box<dyn ImagePipe>>, String> {
    let mut stages = Vec::with_capacity(stage_configs.len());
    println!("Building {} pipeline stages...", stage_configs.len());

    for config in stage_configs {
        let pipe_id = &config.pipe_identifier;
        println!("  - Instantiating pipe: {}", pipe_id);
        let params = &config.parameters; // Alias for convenience

        let pipe_instance: Box<dyn ImagePipe> = match pipe_id.as_str() {
            // --- Resize Pipe ---
            "ResolutionStandardizer" => {
                let target_width = get_param_value(params, "target_width", pipe_id)?
                    .as_u64().ok_or("target_width must be a positive integer")?
                    .try_into().map_err(|_| "'target_width' out of range")?;
                let target_height = get_param_value(params, "target_height", pipe_id)?
                    .as_u64().ok_or("target_height must be a positive integer")?
                    .try_into().map_err(|_| "'target_height' out of range")?;
                let filter_str = get_param_value(params, "filter_type", pipe_id)?
                    .as_str().ok_or("filter_type must be a string")?;

                let filter_type = match filter_str {
                    "Nearest" => FilterType::Nearest,
                    "Triangle" => FilterType::Triangle,
                    "CatmullRom" => FilterType::CatmullRom,
                    "Gaussian" => FilterType::Gaussian,
                    "Lanczos3" => FilterType::Lanczos3,
                    _ => return Err(format!("Invalid filter_type '{}' for {}", filter_str, pipe_id)),
                };

                let pipe = ResizePipe::new(target_width, target_height, filter_type)?;
                Box::new(pipe)
            }
            // --- BlurDetector Pipe (using BlurPipe struct) ---
            "BlurDetector" => {
                let threshold = get_param_value(params, "threshold", pipe_id)?
                    .as_f64().ok_or("threshold must be a number")?;
                let method_str = get_param_value(params, "method", pipe_id)?
                    .as_str().ok_or("method must be a string")?;

                let method = match method_str {
                    "LaplacianVariance" => BlurDetectionMethod::LaplacianVariance,
                    "EdgeIntensity" => BlurDetectionMethod::EdgeIntensity,
                    "PixelVariance" => BlurDetectionMethod::PixelVariance,
                    "EdgeCount" => {
                        let edge_threshold = get_param_value(params, "edge_magnitude_threshold", pipe_id)?
                            .as_u64().ok_or("edge_magnitude_threshold must be a positive integer")?
                            .try_into().map_err(|_| "'edge_magnitude_threshold' out of range")?;
                        BlurDetectionMethod::EdgeCount { edge_magnitude_threshold: edge_threshold }
                    }
                     _ => return Err(format!("Invalid method '{}' for {}", method_str, pipe_id)),
                };

                let pipe = BlurPipe::new(method, threshold)?;
                Box::new(pipe)
            }
            // --- Rotate Pipe ---
            "Rotate" => {
                let angle = get_enum_param::<RotationAngle>(params, "angle", pipe_id)?;
                let pipe = RotatePipe::new(angle);
                Box::new(pipe)
            }
            // --- Flip Pipe ---
            "Flip" => {
                let direction = get_enum_param::<FlipDirection>(params, "direction", pipe_id)?;
                let pipe = FlipPipe::new(direction);
                Box::new(pipe)
            }
            // --- Grayscale Pipe ---
            "Grayscale" => {
                let pipe = GrayscalePipe::new();
                Box::new(pipe)
            }
            // --- Brightness Pipe ---
            "Brightness" => {
                 let value = get_param_value(params, "value", pipe_id)?
                    .as_i64().ok_or("value must be an integer")?
                    .try_into().map_err(|_| "'value' out of range")?;
                let pipe = BrightnessPipe::new(value);
                Box::new(pipe)
            }
            // --- Contrast Pipe ---
            "Contrast" => {
                let contrast = get_param_value(params, "contrast", pipe_id)?
                    .as_f64().ok_or("contrast must be a number")? as f32;
                let pipe = ContrastPipe::new(contrast);
                Box::new(pipe)
            }
            // --- AddNoise Pipe ---
            "AddNoise" => {
                 let noise_type = get_enum_param::<NoiseType>(params, "noise_type", pipe_id)?;
                 match noise_type {
                     NoiseType::Gaussian => {
                         let mean = get_param_value(params, "mean", pipe_id)?
                            .as_f64().ok_or("mean must be a number")?;
                         let std_dev = get_param_value(params, "std_dev", pipe_id)?
                            .as_f64().ok_or("std_dev must be a number")?;
                         let seed = params.get("seed").and_then(|v| v.as_u64());
                         let pipe = AddNoisePipe::new_gaussian(mean, std_dev, seed)?;
                         Box::new(pipe)
                     }
                 }
            }
            // --- NoOp Pipe ---
             "NoOp" => {
                 // Instantiate directly using struct literal
                 let pipe = NoOpStep {};
                 Box::new(pipe)
             }
            // --- Unknown Pipe ---
            _ => {
                return Err(format!(
                    "Unknown pipe identifier in factory: {}",
                    pipe_id
                ));
            }
        };
        stages.push(pipe_instance);
    }
    println!("Finished building stages.");
    Ok(stages)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(OUTPUT_DIR)?;

    println!("Starting pipeline loader example...");
    let overall_start = Instant::now();

    // --- Example Pipeline Configuration ---
    let pipeline_config_stages = vec![
         PipelineStageConfig {
            pipe_identifier: "BlurDetector".to_string(),
            parameters: HashMap::from([
                ("method".to_string(), json!("LaplacianVariance")),
                ("threshold".to_string(), json!(100.0)),
            ]),
        },
        PipelineStageConfig {
            pipe_identifier: "ResolutionStandardizer".to_string(), // Use correct name
            parameters: HashMap::from([
                ("target_width".to_string(), json!(800)),
                ("target_height".to_string(), json!(600)),
                ("filter_type".to_string(), json!("Lanczos3")),
            ]),
        },
        PipelineStageConfig {
            pipe_identifier: "Grayscale".to_string(),
            parameters: HashMap::new(),
        },
         PipelineStageConfig {
            pipe_identifier: "Contrast".to_string(),
            parameters: HashMap::from([
                ("contrast".to_string(), json!(25.5)),
            ]),
        },
        PipelineStageConfig {
            pipe_identifier: "AddNoise".to_string(),
            parameters: HashMap::from([
                ("noise_type".to_string(), json!("gaussian")),
                ("mean".to_string(), json!(0.0)),
                ("std_dev".to_string(), json!(20.0)),
                ("seed".to_string(), json!(42)),
            ]),
        },
         PipelineStageConfig {
            pipe_identifier: "Rotate".to_string(),
            parameters: HashMap::from([
                ("angle".to_string(), json!("rotate90")),
            ]),
        },
        PipelineStageConfig {
            pipe_identifier: "Flip".to_string(),
            parameters: HashMap::from([
                ("direction".to_string(), json!("horizontal")),
            ]),
        },
         PipelineStageConfig {
            pipe_identifier: "Brightness".to_string(),
            parameters: HashMap::from([
                ("value".to_string(), json!(-20)),
            ]),
        },
         PipelineStageConfig {
            pipe_identifier: "NoOp".to_string(),
            parameters: HashMap::new(),
        },
    ];

    println!("Building pipeline stages...");
    let pipeline_stages: Vec<Box<dyn ImagePipe>> = build_pipeline_stages(&pipeline_config_stages)
        .map_err(|e| -> Box<dyn std::error::Error> {
        format!("Failed to build pipeline stages: {}", e).into()
    })?;
    println!(
        "Successfully built {} pipeline stages.",
        pipeline_stages.len()
    );

    // --- Define and Run Pipeline ---
    let pipeline_definition = Pipeline {
        id: "example-mvp-pipes-run-03".to_string(), 
        name: "Example MVP Pipe Sequence (Corrected Factory)".to_string(),
        input_source: DataSource::LocalPath(PathBuf::from(INPUT_DIR)),
        output_destination: DataDestination::LocalPath(PathBuf::from(OUTPUT_DIR)),
        stages: pipeline_stages,
    };
    println!("Executing pipeline '{}'...", pipeline_definition.name);
    runner::run_pipeline(&pipeline_definition).await?;
    println!("Pipeline executed successfully.");

    let total_duration = overall_start.elapsed();
    println!("Example loader finished in {:?}", total_duration);
    println!("Output images saved to '{}' directory.", OUTPUT_DIR);

    Ok(())
}
