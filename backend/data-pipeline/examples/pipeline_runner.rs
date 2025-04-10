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
use data_pipeline::pipeline::{
    ChannelInput,
    ChannelOutput,
    ImagePipe,
    PipeConfig,
    PipeError, // Channel types
    PipeImageData,
    SharedPipelineState,
};
use data_pipeline::pipes::blur::BlurDetectorPipe;
use data_pipeline::pipes::resolution::ResolutionStandardizerPipe;
use data_pipeline::utils::log_pipe_event;

use futures::future::join_all;
use image::{GenericImageView, ImageFormat, ImageReader}; // Added ImageFormat
use serde_json::json;
use tokio::sync::{mpsc, Mutex};
use tokio::task::JoinHandle;

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

// --- Configuration ---
const INPUT_DIR: &str = "test_images";
const OUTPUT_DIR: &str = "output";
const CHANNEL_BUFFER_SIZE: usize = 100; // How many items can queue between stages

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting streaming pipeline runner shim (Tokio tasks + Channels)...");

    // --- Pipeline Stages ---
    let res_pipe = Arc::new(ResolutionStandardizerPipe);
    let blur_pipe = Arc::new(BlurDetectorPipe);

    let res_config = Arc::new(PipeConfig {
        parameters: HashMap::from([
            ("target_width".to_string(), json!(300)),
            ("target_height".to_string(), json!(200)),
            ("filter_type".to_string(), json!("Lanczos3")),
        ]),
    });
    let blur_config = Arc::new(PipeConfig {
        parameters: HashMap::from([
            ("detection_method".to_string(), json!("laplacian_variance")), // Example: Laplacian
            ("laplacian_threshold".to_string(), json!(100.0)),
        ]),
    });

    // Order matters: output of stage i goes to input of stage i+1
    let pipeline_stages: Vec<(Arc<dyn ImagePipe>, Arc<PipeConfig>)> = vec![
        // Stage 0: Blur detection first
        (blur_pipe.clone(), blur_config.clone()),
        // Stage 1: Resolution standardization second
        (res_pipe.clone(), res_config.clone()),
    ];

    // --- Optional: Shared State ---
    let shared_state: Option<SharedPipelineState> = None;

    // --- Setup ---
    fs::create_dir_all(OUTPUT_DIR)?;
    println!("Output directory '{}' ensured.", OUTPUT_DIR);
    let input_path = PathBuf::from(INPUT_DIR);
    let output_path = Arc::new(PathBuf::from(OUTPUT_DIR));

    // --- Create Channels (Revised Logic) ---
    let num_stages = pipeline_stages.len();
    let mut senders = Vec::with_capacity(num_stages + 1);
    let mut receivers = Vec::with_capacity(num_stages + 1);

    // Create N+1 channels for N stages (loader -> p1 -> ... -> pN -> saver)
    for i in 0..=num_stages {
        let (tx, rx) = mpsc::channel::<ChannelInput>(CHANNEL_BUFFER_SIZE);
        // println!("Created channel pair {}", i); // Debug print (optional)
        senders.push(tx);
        receivers.push(rx);
    }

    // Convert receivers Vec into an iterator we can consume safely
    let mut rx_iter = receivers.into_iter();

    // --- Spawn Tasks ---
    let mut handles: Vec<JoinHandle<()>> = Vec::new();
    let start_time = Instant::now();

    // 1. Loader Task
    let loader_tx = senders[0].clone(); // Loader sends to channel 0
    let loader_handle = tokio::spawn(async move {
        log_pipe_event("LOADER", "TASK_INIT", "INFO", "Loader task started.");
        let mut loaded_count = 0;
        let mut load_errors = 0;
        match fs::read_dir(input_path) {
            Ok(entries) => {
                for entry in entries.filter_map(Result::ok) {
                    let path = entry.path();
                    if path.is_file() {
                        let filename = path.file_name().unwrap_or_default().to_string_lossy();
                        let image_id = filename.to_string();

                        // Load image logic
                        let reader_result = ImageReader::open(&path)
                            .map_err(|e| format!("Failed open: {}", e))
                            .and_then(|r| {
                                r.with_guessed_format()
                                    .map_err(|e| format!("Failed guess format: {}", e))
                            });

                        let item_to_send: ChannelInput = match reader_result {
                            Ok(reader) => {
                                let image_format = reader.format().unwrap_or(ImageFormat::Png);
                                match reader.decode() {
                                    Ok(image) => {
                                        let metadata = HashMap::from([
                                            (
                                                "original_filename".to_string(),
                                                json!(image_id.clone()),
                                            ),
                                            ("original_width".to_string(), json!(image.width())),
                                            ("original_height".to_string(), json!(image.height())),
                                        ]);
                                        loaded_count += 1;
                                        Ok(PipeImageData {
                                            id: image_id,
                                            image,
                                            metadata,
                                            original_format: image_format,
                                        })
                                    }
                                    Err(e) => {
                                        load_errors += 1;
                                        Err(format!("Failed decode [{}]: {}", image_id, e))
                                    }
                                }
                            }
                            Err(e) => {
                                load_errors += 1;
                                Err(format!("Failed read [{}]: {}", image_id, e))
                            }
                        };
                        // Send loaded item or error
                        if loader_tx.send(item_to_send).await.is_err() {
                            log_pipe_event(
                                "LOADER",
                                "TASK_EXIT",
                                "WARN",
                                "Output channel closed. Stopping load.",
                            );
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                log_pipe_event(
                    "LOADER",
                    "TASK_INIT",
                    "ERROR",
                    &format!("Failed to read input directory: {}", e),
                );
            }
        }
        log_pipe_event(
            "LOADER",
            "TASK_EXIT",
            "INFO",
            &format!(
                "Loader finished. Loaded: {}, Errors: {}",
                loaded_count, load_errors
            ),
        );
        // Sender drops here, closing channel 0
    });
    handles.push(loader_handle);

    // 2. Pipe Stage Tasks
    for i in 0..num_stages {
        let pipe = Arc::clone(&pipeline_stages[i].0);
        let config = Arc::clone(&pipeline_stages[i].1);
        let state = shared_state.clone();
        // Take the next receiver from the iterator for this stage's input
        let mut input_rx = rx_iter
            .next()
            .expect(&format!("Should have receiver for stage {}", i));
        // Get the sender for this stage's output (next stage's input)
        let output_tx = senders[i + 1].clone();

        let stage_handle = tokio::spawn(async move {
            pipe.run_stage(config, state, &mut input_rx, output_tx)
                .await;
            // Receiver input_rx is dropped here
        });
        handles.push(stage_handle);
    }

    // 3. Saver/Collector Task
    // Take the final receiver from the iterator
    let mut final_rx = rx_iter.next().expect("Should have receiver for saver task");
    let processed_count = Arc::new(AtomicUsize::new(0));
    let error_count = Arc::new(AtomicUsize::new(0));

    let p_count = Arc::clone(&processed_count);
    let e_count = Arc::clone(&error_count);
    let out_path = Arc::clone(&output_path);

    let saver_handle = tokio::spawn(async move {
        log_pipe_event("SAVER", "TASK_INIT", "INFO", "Saver task started.");
        while let Some(result) = final_rx.recv().await {
            match result {
                Ok(final_data) => {
                    // Save logic
                    let image_id = final_data.id.clone();
                    let output_filename = format!(
                        "processed_{}",
                        final_data.metadata["original_filename"]
                            .as_str()
                            .unwrap_or(&image_id)
                    );
                    let output_filepath = out_path.join(&output_filename);
                    let save_result = tokio::task::spawn_blocking(move || {
                        final_data
                            .image
                            .save_with_format(&output_filepath, final_data.original_format)
                    })
                    .await;
                    match save_result {
                        Ok(Ok(_)) => {
                            p_count.fetch_add(1, Ordering::Relaxed);
                        }
                        Ok(Err(e)) => {
                            log_pipe_event(
                                "SAVER",
                                &image_id,
                                "ERROR",
                                &format!("Failed save: {}", e),
                            );
                            e_count.fetch_add(1, Ordering::Relaxed);
                        }
                        Err(join_err) => {
                            log_pipe_event(
                                "SAVER",
                                &image_id,
                                "ERROR",
                                &format!("Save task failed: {}", join_err),
                            );
                            e_count.fetch_add(1, Ordering::Relaxed);
                        }
                    }
                }
                Err(pipe_err) => {
                    log_pipe_event(
                        "SAVER",
                        "UNKNOWN_ID",
                        "ERROR",
                        &format!("Received error: {}", pipe_err),
                    );
                    e_count.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
        log_pipe_event("SAVER", "TASK_EXIT", "INFO", "Saver finished.");
        // Receiver final_rx drops here
    });
    handles.push(saver_handle);
    drop(senders);

    // --- Wait for all tasks to complete ---
    println!("Waiting for all pipeline tasks to complete...");
    join_all(handles).await; // Wait for loader, pipes, saver
    println!("All tasks finished.");

    // Stop timer
    let duration = start_time.elapsed();
    println!("Streaming pipeline finished in: {:?}", duration);

    // --- Summary ---
    println!("\n--- Pipeline Run Summary ---");
    println!(
        "Processed and Saved: {}",
        processed_count.load(Ordering::Relaxed)
    );
    // Discard counts are logged by pipes, not easily aggregated here without more channels/state
    println!(
        "Errors (Load+Pipe+Save): {}",
        error_count.load(Ordering::Relaxed)
    );
    println!("--------------------------");
    println!("Total processing time: {:?}", duration);
    println!("--------------------------");

    Ok(())
}
