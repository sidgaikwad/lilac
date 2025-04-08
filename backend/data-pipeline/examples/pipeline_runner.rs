//! Example runner for testing image processing pipes locally.
//!
//! Usage:
//! 1. Place test images (JPEG, PNG) in the `backend/data-pipeline/test_images/` directory.
//! 2. Create an empty `backend/data-pipeline/output/` directory.
//! 3. Run this example from the `backend/data-pipeline` directory using:
//!    `cargo run --example pipeline_runner`
//!
//! To add more pipes:
//! 1. Instantiate the pipe struct (e.g., `let blur_pipe = BlurDetectorPipe;`).
//! 2. Define its `PipeConfig` (e.g., `let blur_config = PipeConfig { ... };`).
//! 3. Add the pipe and its config to the `pipeline_stages` vector in the desired order.

// --- Imports ---
use data_pipeline::pipeline::{
    ImagePipe, PipeImageData, PipeResult, PipeConfig, ImageMetadata,
};
use data_pipeline::pipes::resolution::ResolutionStandardizerPipe; // Import the specific pipe
use data_pipeline::utils::log_pipe_event;

use image::{ImageFormat, DynamicImage, ImageReader}; // Use non-deprecated ImageReader path
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// --- Configuration ---
const INPUT_DIR: &str = "test_images";
const OUTPUT_DIR: &str = "output";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting pipeline runner shim...");

    // --- Define the Pipeline ---
    let res_pipe = ResolutionStandardizerPipe;
    // TODO: Instantiate other pipes here

    let res_config = PipeConfig {
        parameters: HashMap::from([
            ("target_width".to_string(), json!(3000)),
            ("target_height".to_string(), json!(2000)),
            ("filter_type".to_string(), json!("Lanczos3")),
        ]),
    };
    // TODO: Define configs for other pipes here

    // Define the sequence of pipes and their configs
    let pipeline_stages: Vec<(&dyn ImagePipe, &PipeConfig)> = vec![
        (&res_pipe, &res_config),
        // TODO: Add other pipes and configs to the vector in order
    ];

    // --- Setup ---
    fs::create_dir_all(OUTPUT_DIR)?;
    println!("Output directory '{}' ensured.", OUTPUT_DIR);
    let input_path = Path::new(INPUT_DIR);
    let output_path = Path::new(OUTPUT_DIR);

    // --- Process Images ---
    println!("Scanning input directory '{}'...", INPUT_DIR);
    let mut processed_count = 0;
    let mut discarded_count = 0;
    let mut error_count = 0;

    for entry in fs::read_dir(input_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let filename = path.file_name().unwrap_or_default().to_string_lossy();
            let image_id = filename.to_string();
            println!("\n--- Processing Image: {} ---", image_id);

            // Load Image using ImageReader
            let reader = match ImageReader::open(&path) {
                Ok(r) => r,
                Err(e) => {
                    log_pipe_event("LOADER", &image_id, "ERROR", &format!("Failed to open: {}", e));
                    error_count += 1;
                    continue;
                }
            };
            let reader = match reader.with_guessed_format() {
                 Ok(r) => r,
                 Err(e) => {
                    log_pipe_event("LOADER", &image_id, "ERROR", &format!("Failed guess format: {}", e));
                    error_count += 1;
                    continue;
                }
            };
            let image_format = reader.format().unwrap_or(ImageFormat::Png);
            log_pipe_event("LOADER", &image_id, "DEBUG", &format!("Detected format: {:?}", image_format));
            let current_image: DynamicImage = match reader.decode() {
                Ok(img) => {
                    log_pipe_event("LOADER", &image_id, "INFO", "Image decoded.");
                    img
                }
                Err(e) => {
                    log_pipe_event("LOADER", &image_id, "ERROR", &format!("Failed decode: {}", e));
                    error_count += 1;
                    continue;
                }
            };

            // Create initial PipeImageData
            let initial_metadata: ImageMetadata = HashMap::from([
                ("original_filename".to_string(), json!(image_id.clone())),
                ("original_width".to_string(), json!(current_image.width())),
                ("original_height".to_string(), json!(current_image.height())),
            ]);
            let pipe_data = PipeImageData {
                id: image_id.clone(),
                image: current_image,
                metadata: initial_metadata,
                original_format: image_format,
            };

            let mut current_result: Option<PipeImageData> = Some(pipe_data);

            // Run through pipeline stages
            for (pipe, config) in &pipeline_stages {
                if let Some(data_to_process) = current_result.take() {
                    let result = pipe.process(data_to_process, config).await;
                    match result {
                        PipeResult::Modified(new_data) | PipeResult::Unchanged(new_data) => {
                            current_result = Some(new_data);
                        }
                        PipeResult::Discarded { reason } => {
                            log_pipe_event(pipe.name(), &image_id, "INFO", &format!("Discarded: {}", reason));
                            discarded_count += 1;
                            current_result = None;
                            break;
                        }
                        PipeResult::Error { message } => {
                            log_pipe_event(pipe.name(), &image_id, "ERROR", &format!("Error: {}", message));
                            error_count += 1;
                            current_result = None;
                            break;
                        }
                    }
                } else {
                    log_pipe_event("RUNNER", &image_id, "WARN", "Data missing mid-pipeline.");
                    break;
                }
            }

            // Save final image if it wasn't discarded/errored
            if let Some(final_data) = current_result {
                let output_filename = format!("processed_{}", filename);
                let output_filepath = output_path.join(&output_filename);
                log_pipe_event("SAVER", &image_id, "INFO", &format!("Saving to: {:?}", output_filepath));
                match final_data.image.save_with_format(&output_filepath, final_data.original_format) {
                    Ok(_) => {
                        processed_count += 1;
                    }
                    Err(e) => {
                        log_pipe_event("SAVER", &image_id, "ERROR", &format!("Failed save: {}", e));
                        error_count += 1;
                    }
                }
            }
        }
    } // End of loop through directory entries

    // --- Summary ---
    println!("\n--- Pipeline Run Summary ---");
    println!("Processed and Saved: {}", processed_count);
    println!("Discarded:           {}", discarded_count);
    println!("Errors:              {}", error_count);
    println!("--------------------------");

    Ok(())
}
