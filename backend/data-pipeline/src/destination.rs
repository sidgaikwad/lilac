//! src/destination.rs
//! Defines and implements logic for saving processed image data to various destinations.

use crate::pipe_core::PipeImageData;
use crate::pipeline_definition::DataDestination;
use crate::utils::log_pipe_event;
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DestinationError {
    #[error("I/O Error interacting with destination: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image saving error: {0}")]
    ImageSaveError(String),
    #[error("Unsupported data destination type specified")]
    UnsupportedDestination,
    #[error("Failed to save {count} images to destination")]
    PartialSave { count: u32 },
}

pub fn save_batch(batch: &[PipeImageData], dest: &DataDestination) -> Result<(), DestinationError> {
    match dest {
        DataDestination::LocalPath(path) => save_to_local_path(batch, path),
        // Example for future S3 implementation
        // DataDestination::S3Bucket { bucket, prefix } => save_to_s3(batch, bucket, prefix),
        // Add other destination types here
        // _ => Err(DestinationError::UnsupportedDestination),
    }
}

fn save_to_local_path(batch: &[PipeImageData], path: &Path) -> Result<(), DestinationError> {
    let destination_id = path.display().to_string();
    log_pipe_event(
        "Destination",
        &destination_id,
        "INFO",
        &format!("Starting save to local path: {:?}", path),
    );

    fs::create_dir_all(path)?;

    let mut save_errors: u32 = 0;

    for img_data in batch {
        let original_filename = img_data
            .metadata
            .get("original_filename")
            .and_then(|v| v.as_str())
            .unwrap_or(&img_data.id);
        let output_filename = format!("processed_{}", original_filename);
        let output_filepath = path.join(&output_filename);

        match img_data
            .image
            .save_with_format(&output_filepath, img_data.original_format)
        {
            Ok(_) => {}
            Err(e) => {
                log_pipe_event(
                    "Destination",
                    &img_data.id,
                    "ERROR",
                    &format!("Failed to save to {:?}: {}", output_filepath, e),
                );
                save_errors += 1;
            }
        }
    }

    log_pipe_event(
        "Destination",
        &destination_id,
        "INFO",
        &format!(
            "Finished saving. Success: {}, Errors: {}",
            batch.len() as u32 - save_errors,
            save_errors
        ),
    );

    if save_errors > 0 {
        Err(DestinationError::PartialSave { count: save_errors })
    } else {
        Ok(())
    }
}

// TO-DO: S3 saver
// Placeholder for S3 saving function
// fn save_to_s3(batch: &[PipeImageData], bucket: &str, prefix: &str) -> Result<(), DestinationError> {
//     // ... implementation using rusoto_s3 or aws-sdk-s3 ...
//     // Might involve parallel uploads for efficiency
//     unimplemented!("S3 saving not implemented yet");
// }
