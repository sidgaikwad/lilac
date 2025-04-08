//! Example runner for testing image processing pipes locally using Rayon for parallelism.
//! This version captures the Tokio runtime handle to run async pipe functions.
//!
//! Usage:
//! 1. Place test images (JPEG, PNG) in the `backend/data-pipeline/test_images/` directory.
//! 2. Create an empty `backend/data-pipeline/output/` directory.
//! 3. Run this example from the `backend/data-pipeline` directory using:
//!    `cargo run --example pipeline_runner`
//!
//! To add more pipes:
//! 1. Instantiate the pipe struct (use Arc).
//! 2. Define its `PipeConfig` (use Arc).
//! 3. Add the Arc'd pipe and config to the `pipeline_stages` vector in the desired order.

// --- Imports ---
use data_pipeline::pipeline::{
    ImagePipe, PipeImageData, PipeResult, PipeConfig, ImageMetadata,
};
use data_pipeline::pipes::resolution::ResolutionStandardizerPipe;
use data_pipeline::utils::log_pipe_event;

use image::{ImageFormat, DynamicImage, ImageReader};
use rayon::prelude::*;
use serde_json::json;
use tokio::runtime::Handle;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

// --- Configuration ---
const INPUT_DIR: &str = "test_images";
const OUTPUT_DIR: &str = "output";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting parallel pipeline runner shim (Rayon + Captured Handle)...");

    // --- Define the Pipeline ---
    let res_pipe = Arc::new(ResolutionStandardizerPipe);
    // TODO: Instantiate other pipes here (use Arc)

    let res_config = Arc::new(PipeConfig {
        parameters: HashMap::from([
            ("target_width".to_string(), json!(300)),
            ("target_height".to_string(), json!(200)),
            ("filter_type".to_string(), json!("Lanczos3")),
        ]),
    });
    // TODO: Define configs for other pipes here (use Arc)

    let pipeline_stages: Arc<Vec<(Arc<dyn ImagePipe>, Arc<PipeConfig>)>> = Arc::new(vec![
        (res_pipe.clone(), res_config.clone()),
        // TODO: Add other Arc'd pipes and configs here
    ]);

    // --- Setup ---
    fs::create_dir_all(OUTPUT_DIR)?;
    println!("Output directory '{}' ensured.", OUTPUT_DIR);
    let input_path = Path::new(INPUT_DIR);
    let output_path = Arc::new(PathBuf::from(OUTPUT_DIR));

    // --- Collect Image Paths ---
    println!("Scanning input directory '{}'...", INPUT_DIR);
    let image_paths: Vec<PathBuf> = fs::read_dir(input_path)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();

    if image_paths.is_empty() {
        println!("No image files found in {}.", INPUT_DIR);
        return Ok(());
    }
    println!("Found {} potential image files.", image_paths.len());

    // --- Process Images in Parallel using Rayon ---
    println!("Processing images using Rayon...");
    let processed_count = AtomicUsize::new(0);
    let discarded_count = AtomicUsize::new(0);
    let error_count = AtomicUsize::new(0);

    // Get the handle to the current Tokio runtime *before* the parallel iteration
    let rt = Handle::current();

    // Start timer
    let start_time = Instant::now();

    image_paths.par_iter().for_each(|path| {
        // Clone the handle for use in this specific Rayon task
        let rt_clone = rt.clone();

        let filename = path.file_name().unwrap_or_default().to_string_lossy();
        let image_id = filename.to_string();

        // --- Load Image ---
        let reader = match ImageReader::open(path) {
            Ok(r) => r,
            Err(e) => {
                log_pipe_event("LOADER", &image_id, "ERROR", &format!("Failed open: {}", e));
                error_count.fetch_add(1, Ordering::Relaxed);
                return;
            }
        };
        let reader = match reader.with_guessed_format() {
             Ok(r) => r,
             Err(e) => {
                log_pipe_event("LOADER", &image_id, "ERROR", &format!("Failed guess format: {}", e));
                error_count.fetch_add(1, Ordering::Relaxed);
                return;
            }
        };
        let image_format = reader.format().unwrap_or(ImageFormat::Png);
        let current_image: DynamicImage = match reader.decode() {
            Ok(img) => img,
            Err(e) => {
                log_pipe_event("LOADER", &image_id, "ERROR", &format!("Failed decode: {}", e));
                error_count.fetch_add(1, Ordering::Relaxed);
                return;
            }
        };

        // --- Create initial PipeImageData ---
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

        // --- Run through pipeline stages ---
        // Clone Arcs needed inside the loop/closure
        let pipeline_stages_clone = Arc::clone(&pipeline_stages);
        for (pipe, config) in pipeline_stages_clone.iter() {
            if let Some(data_to_process) = current_result.take() {
                 // Use the cloned handle's block_on to run the async process function
                 // This blocks the current Rayon thread until the async function completes
                 let result = rt_clone.block_on(
                     // Pass arguments to pipe
                     pipe.process(data_to_process, config)
                 );

                match result {
                    PipeResult::Modified(new_data) | PipeResult::Unchanged(new_data) => {
                        current_result = Some(new_data);
                    }
                    PipeResult::Discarded { reason } => {
                        log_pipe_event(pipe.name(), &image_id, "INFO", &format!("Discarded: {}", reason));
                        discarded_count.fetch_add(1, Ordering::Relaxed);
                        current_result = None;
                        break;
                    }
                    PipeResult::Error { message } => {
                        log_pipe_event(pipe.name(), &image_id, "ERROR", &format!("Error: {}", message));
                        error_count.fetch_add(1, Ordering::Relaxed);
                        current_result = None;
                        break;
                    }
                }
            } else {
                log_pipe_event("RUNNER_TASK", &image_id, "WARN", "Data missing mid-pipeline.");
                break;
            }
        } // End pipeline stages loop

        // --- Save final image ---
        if let Some(final_data) = current_result {
            let output_filename = format!("processed_{}", filename);
            // Clone Arc'd output_path for saving
            let output_filepath = Arc::clone(&output_path).join(&output_filename);
            match final_data.image.save_with_format(&output_filepath, final_data.original_format) {
                Ok(_) => {
                    processed_count.fetch_add(1, Ordering::Relaxed);
                }
                Err(e) => {
                    log_pipe_event("SAVER", &image_id, "ERROR", &format!("Failed save: {}", e));
                    error_count.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
    }); // End of par_iter().for_each()

    let duration = start_time.elapsed();

    // --- Summary ---
    println!("\n--- Pipeline Run Summary ---");
    println!("Processed and Saved: {}", processed_count.load(Ordering::Relaxed));
    println!("Discarded:           {}", discarded_count.load(Ordering::Relaxed));
    println!("Errors:              {}", error_count.load(Ordering::Relaxed));
    println!("--------------------------");
    println!("Parallel processing finished in: {:?}", duration);
    println!("--------------------------");

    Ok(())
}
