//! Example runner demonstrating a streaming pipeline using Tokio tasks and channels.
//! Each pipe runs in its own task, processing images as they arrive.
//!
//! Usage:
//! 1. Place test images (JPEG, PNG) in the `backend/data-pipeline/test_images/` directory.
//! 2. Create an empty `backend/data-pipeline/output/` directory.
//! 3. Ensure `tokio` (with features) and `futures` are in Cargo.toml dependencies.
//! 4. Run this example from the `backend/data-pipeline` directory using:
//!    `cargo run --example pipeline_runner`

// --- Imports ---
use data_pipeline::pipeline::{ImagePipe, PipeImageData};
use data_pipeline::pipes::blur::{BlurDetectionMethod, BlurDetectorPipe};
use data_pipeline::pipes::resolution::ResolutionStandardizerPipe;

use image::imageops::FilterType;
use image::{ImageFormat, ImageReader}; // Added ImageFormat
use serde_json::json;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;

// --- Configuration ---
const INPUT_DIR: &str = "test_images";
const OUTPUT_DIR: &str = "output";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting streaming pipeline runner shim (Tokio tasks + Channels)...");

    // --- Pipeline Stages ---
    let res_pipe = ResolutionStandardizerPipe::new(300, 200, FilterType::Lanczos3);
    let blur_pipe = BlurDetectorPipe::new(BlurDetectionMethod::LaplacianVariance, 100.0);

    // Order matters: output of stage i goes to input of stage i+1
    let pipeline_stages: Vec<Arc<dyn ImagePipe>> = vec![Arc::new(res_pipe), Arc::new(blur_pipe)];

    // --- Setup ---
    fs::create_dir_all(OUTPUT_DIR)?;
    println!("Output directory '{}' ensured.", OUTPUT_DIR);
    let input_path = PathBuf::from(INPUT_DIR);
    let output_path = Arc::new(PathBuf::from(OUTPUT_DIR));

    let start_time = Instant::now();

    // Load images
    let dir = fs::read_dir(input_path).expect("input dir to exist");
    let image_paths = dir
        .filter_map(Result::ok)
        .map(|f| f.path())
        .filter(|f| f.is_file());

    let mut images = Vec::new();
    for image_path in image_paths {
        let filename = image_path.file_name().unwrap_or_default().to_string_lossy();
        let r = ImageReader::open(&image_path)
            .and_then(|r| r.with_guessed_format())
            .expect("image to be read");
        let format = r.format().unwrap_or(ImageFormat::Png);
        let image = r.decode();
        match image {
            Ok(img) => {
                let image_data = PipeImageData {
                    id: filename.to_string(),
                    image: img,
                    metadata: HashMap::from([(
                        "original_filename".to_string(),
                        json!(filename.to_string()),
                    )]),
                    original_format: format,
                };

                images.push(image_data);
            }
            Err(_) => continue,
        }
    }

    // 2. Pipe Stage Tasks
    for pipe in pipeline_stages {
        images = pipe.run_stage(images).await?;
    }

    // 3. Save outputs
    // Save logic
    for img in images {
        let output_filename = format!(
            "processed_{}",
            img.metadata["original_filename"].as_str().unwrap()
        );
        let output_filepath = output_path.join(&output_filename);
        img.image
            .save_with_format(&output_filepath, img.original_format)
            .unwrap();
    }

    // Stop timer
    let duration = start_time.elapsed();
    println!("Streaming pipeline finished in: {:?}", duration);

    // --- Summary ---
    println!("\n--- Pipeline Run Summary ---");

    println!("--------------------------");
    println!("Total processing time: {:?}", duration);
    println!("--------------------------");

    Ok(())
}
