//! examples/pipeline_loader.rs
//! Demonstrates defining a pipeline configuration, building the runnable
//! pipeline instance (instantiating pipes via factory logic), and executing
//! it using the runner module.

use data_pipeline::pipe_core::{ImagePipe, PipelineStageConfig, PipeError}; // PipeError might be needed if new returns Result
use data_pipeline::pipeline_definition::{Pipeline, DataSource, DataDestination};
use data_pipeline::runner;
use data_pipeline::pipes::blur::{BlurDetectorPipe, BlurDetectionMethod};
use data_pipeline::pipes::resolution::ResolutionStandardizerPipe;
use image::imageops::FilterType;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Instant;

const INPUT_DIR: &str = "test_images";
const OUTPUT_DIR: &str = "output";

fn build_pipeline_stages(
    stage_configs: &[PipelineStageConfig],
) -> Result<Vec<Arc<dyn ImagePipe>>, String> {
    let mut stages = Vec::with_capacity(stage_configs.len());
    println!("Building {} pipeline stages...", stage_configs.len());

    for config in stage_configs {
        println!("  - Instantiating pipe: {}", config.pipe_identifier);
        let params = &config.parameters; // Alias for convenience

        let pipe_instance: Arc<dyn ImagePipe> = match config.pipe_identifier.as_str() {
            "ResolutionStandardizer" => {
                let target_width = params.get("target_width")
                    .and_then(Value::as_u64).map(|v| v as u32)
                    .ok_or_else(|| format!("Missing/invalid 'target_width' for {}", config.pipe_identifier))?;
                let target_height = params.get("target_height")
                    .and_then(Value::as_u64).map(|v| v as u32)
                    .ok_or_else(|| format!("Missing/invalid 'target_height' for {}", config.pipe_identifier))?;
                let filter_str = params.get("filter_type")
                    .and_then(Value::as_str).unwrap_or("Lanczos3");
                let filter_type = match filter_str.to_lowercase().as_str() {
                    "nearest" => FilterType::Nearest,
                    "triangle" => FilterType::Triangle,
                    "catmullrom" => FilterType::CatmullRom,
                    "gaussian" => FilterType::Gaussian,
                    "lanczos3" => FilterType::Lanczos3,
                    _ => return Err(format!("Invalid filter_type '{}' for {}", filter_str, config.pipe_identifier)),
                };
                 if target_width == 0 || target_height == 0 {
                     return Err(format!("Target width/height must be > 0 for {}", config.pipe_identifier));
                 }

                let pipe = ResolutionStandardizerPipe::new(target_width, target_height, filter_type);
                Arc::new(pipe)
            }
            "BlurDetector" => {
                let method_str = params.get("detection_method")
                    .and_then(Value::as_str).unwrap_or("laplacian_variance");
                let threshold_key = match method_str {
                     "laplacian_variance" => "laplacian_threshold",
                     "edge_intensity" => "edge_intensity_threshold",
                     "pixel_variance" => "pixel_variance_threshold",
                     "edge_count" => "edge_count_threshold",
                     _ => return Err(format!("Invalid detection_method '{}' for {}", method_str, config.pipe_identifier)),
                 };
                 let threshold = params.get(threshold_key)
                     .and_then(Value::as_f64)
                     .ok_or_else(|| format!("Missing/invalid threshold '{}' for {}", threshold_key, config.pipe_identifier))?;
                 if threshold < 0.0 { return Err(format!("Threshold cannot be negative for {}", config.pipe_identifier)); }

                 let method = match method_str {
                     "laplacian_variance" => BlurDetectionMethod::LaplacianVariance,
                     "edge_intensity" => BlurDetectionMethod::EdgeIntensity,
                     "pixel_variance" => BlurDetectionMethod::PixelVariance,
                     "edge_count" => {
                         let mag_threshold = params.get("edge_magnitude_threshold")
                             .and_then(Value::as_u64).map(|v| v as u16)
                             .ok_or_else(|| format!("Missing/invalid 'edge_magnitude_threshold' for EdgeCount method in {}", config.pipe_identifier))?;
                         BlurDetectionMethod::EdgeCount { edge_magnitude_threshold: mag_threshold }
                     },
                     _ => unreachable!(),
                 };

                let pipe = BlurDetectorPipe::new(method, threshold);
                Arc::new(pipe)
            }
            _ => {
                return Err(format!(
                    "Unknown pipe identifier in factory: {}",
                    config.pipe_identifier
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
    println!("Starting pipeline loader example...");
    let overall_start = Instant::now();

    let pipeline_config_stages = vec![
        PipelineStageConfig {
            pipe_identifier: "BlurDetector".to_string(),
            parameters: HashMap::from([
                ("detection_method".to_string(), json!("laplacian_variance")),
                ("laplacian_threshold".to_string(), json!(100.0)),
            ]),
        },
        PipelineStageConfig {
            pipe_identifier: "ResolutionStandardizer".to_string(),
            parameters: HashMap::from([
                ("target_width".to_string(), json!(500)),
                ("target_height".to_string(), json!(500)),
                ("filter_type".to_string(), json!("Triangle")),
            ]),
        },
    ];

    println!("Building pipeline stages...");
    let pipeline_stages: Vec<Arc<dyn ImagePipe>> = build_pipeline_stages(&pipeline_config_stages)
        .map_err(|e| -> Box<dyn std::error::Error> {
            format!("Failed to build pipeline stages: {}", e).into()
        })?;
    println!("Successfully built {} pipeline stages.", pipeline_stages.len());


    let pipeline_definition = Pipeline {
        id: "example-local-run-01".to_string(),
        name: "Example Blur then Resize 500x500".to_string(),
        input_source: DataSource::LocalPath(PathBuf::from(INPUT_DIR)),
        output_destination: DataDestination::LocalPath(PathBuf::from(OUTPUT_DIR)),
        stages: pipeline_stages,
    };
    println!("Executing pipeline '{}'...", pipeline_definition.name);
    runner::run_pipeline(&pipeline_definition).await?;
    println!("Pipeline executed successfully."); 
    let total_duration = overall_start.elapsed();
    println!("Example loader finished in {:?}", total_duration);
    Ok(())
}

