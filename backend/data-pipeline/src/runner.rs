//! src/runner.rs
//! Orchestrates the execution of a defined pipeline by calling
//! abstracted data source/destination functions and executing pre-instantiated pipe stages.

use crate::pipe_core::PipeError; // Core trait and data
use crate::pipeline_definition::Pipeline; // Pipeline definition struct
use crate::utils::log_pipe_event;
use crate::{datasource, destination}; // Import the I/O modules // Logging utility

use std::sync::Arc;
use std::time::Instant;
use common::database::Database;
use common::s3::S3Wrapper;
use thiserror::Error; // For structured errors

/// Errors that can occur during pipeline execution orchestration.
#[derive(Error, Debug)]
pub enum RunnerError {
    #[error("DataSource Error: {0}")]
    Source(#[from] datasource::SourceError), // Wrap SourceError
    #[error("Destination Error: {0}")]
    Destination(#[from] destination::DestinationError), // Wrap DestinationError
    #[error("Pipe Execution Error in stage '{stage_name}': {source}")]
    PipeExecution {
        stage_name: String,
        #[source] // Allows fetching the underlying PipeError
        source: PipeError,
    },
}

/// Executes a defined pipeline containing pre-instantiated stages.
/// Input: Reference to the Pipeline definition.
/// Output: Result indicating overall success or the first encountered RunnerError.
pub async fn run_pipeline(db: Arc<Database>, s3: S3Wrapper, pipeline: &Pipeline) -> Result<(), RunnerError> {
    let run_id = &pipeline.id; // Use pipeline ID for context if available
    log_pipe_event(
        "Runner",
        run_id,
        "INFO",
        &format!("Starting pipeline run: {}", pipeline.name),
    );
    let overall_start_time = Instant::now();

    // --- 1. Load Input Images ---
    log_pipe_event(
        "Runner",
        run_id,
        "INFO",
        &format!("Loading data from source"),
    );
    let load_start_time = Instant::now();

    // Call generic load function from the datasource module
    let mut current_batch = datasource::load_batch(&pipeline.input_source).await?; // Propagate SourceError

    let load_duration = load_start_time.elapsed();
    let initial_count = current_batch.len();
    log_pipe_event(
        "Runner",
        run_id,
        "INFO",
        &format!("Loaded {} images in {:?}", initial_count, load_duration),
    );

    if current_batch.is_empty() {
        log_pipe_event(
            "Runner",
            run_id,
            "WARN",
            "Input batch is empty. Pipeline run will complete without processing.",
        );
        // Decide if this should be an error or just finish successfully
    }

    // --- 2. Execute Pipeline Stages Sequentially ---
    log_pipe_event(
        "Runner",
        run_id,
        "INFO",
        &format!("Executing {} pipeline stages...", pipeline.stages.len()),
    );
    let processing_start_time = Instant::now();
    let mut images_in_stage = initial_count; // Track count entering each stage

    for pipe_instance in &pipeline.stages {
        // Iterate Vec<Arc<dyn ImagePipe>>
        let stage_name = pipe_instance.name();
        if current_batch.is_empty() {
            log_pipe_event(
                "Runner",
                run_id,
                "WARN",
                &format!("Skipping stage '{}' - input batch is empty.", stage_name),
            );
            continue;
        }
        log_pipe_event(
            "Runner",
            run_id,
            "INFO",
            &format!(
                "Running stage '{}' on {} images...",
                stage_name,
                current_batch.len()
            ),
        );

        // No factory needed - just call run_stage on the trait object
        let stage_start_time = Instant::now();
        current_batch = pipe_instance.run_stage(current_batch).await.map_err(|e| {
            // Wrap PipeError into RunnerError::PipeExecution
            RunnerError::PipeExecution {
                stage_name: stage_name.to_string(),
                source: e,
            }
        })?; // Propagate error if pipe stage fails
        let stage_duration = stage_start_time.elapsed();
        let images_out_stage = current_batch.len();
        let discarded = images_in_stage.saturating_sub(images_out_stage); // Use saturating_sub

        log_pipe_event(
            "Runner",
            run_id,
            "INFO",
            &format!(
                "Finished stage '{}' in {:?}. Output: {} images ({} discarded)",
                stage_name, stage_duration, images_out_stage, discarded
            ),
        );
        images_in_stage = images_out_stage; // Update count for next stage input
    }
    let processing_duration = processing_start_time.elapsed();
    log_pipe_event(
        "Runner",
        run_id,
        "INFO",
        &format!("Finished all stages in {:?}", processing_duration),
    );

    // --- 3. Save Output Images ---
    log_pipe_event(
        "Runner",
        run_id,
        "INFO",
        &format!(
            "Saving {} processed images to: {:?}",
            current_batch.len(),
            pipeline.output_destination
        ),
    );
    let save_start_time = Instant::now();

    // Call generic save function from the destination module
    destination::save_batch(db.clone(), s3.clone(), &current_batch, &pipeline.output_destination).await?; // Propagate DestinationError

    let save_duration = save_start_time.elapsed();
    let saved_count = current_batch.len(); // All remaining images were intended for saving
    log_pipe_event(
        "Runner",
        run_id,
        "INFO",
        &format!("Saved {} images in {:?}", saved_count, save_duration),
    );

    // --- Finish ---
    let overall_duration = overall_start_time.elapsed();
    log_pipe_event(
        "Runner",
        run_id,
        "INFO",
        &format!(
            "Pipeline run finished successfully in {:?}.",
            overall_duration
        ),
    );
    println!("\n--- Pipeline Run Summary ---"); // Keep simple summary for example runner
    println!("Initial Images (Loaded): {}", initial_count);
    println!("Final Images (Saved): {}", saved_count);
    // Errors cause early exit, discards are logged per stage
    println!("--------------------------");
    println!("Total processing time: {:?}", overall_duration);
    println!("--------------------------");

    Ok(())
}
