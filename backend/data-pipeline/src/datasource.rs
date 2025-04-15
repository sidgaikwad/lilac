//! src/datasource.rs
//! Defines and implements logic for loading image data from various sources.

use crate::pipe_core::{ImageMetadata, PipeImageData}; // Use the renamed core module
use crate::pipeline_definition::DataSource; // Use definition module for the enum
use crate::utils::log_pipe_event; // Use logging utility

use image::{ImageFormat, ImageReader};
use serde_json::json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use thiserror::Error;

/// Errors that can occur during data loading.
#[derive(Error, Debug)]
pub enum SourceError {
    #[error("I/O Error reading source: {0}")]
    Io(#[from] std::io::Error), // Auto-convert std::io::Error
    #[error("Image processing error in source: {0}")]
    ImageError(#[from] image::ImageError), // Auto-convert image::ImageError
    #[error("Error reading directory entry")]
    ReadDirEntryError,
    #[error("Unsupported data source type specified")]
    UnsupportedSource,
    #[error("Image load failed for {count} images")] // Error for load summary
    ImageLoadFailed { count: u32 },
    #[error("No images successfully loaded from source")]
    NoImagesLoaded,
}

pub fn load_batch(source: &DataSource) -> Result<Vec<PipeImageData>, SourceError> {
    match source {
        DataSource::LocalPath(path) => load_from_local_path(path),
        // Example for future S3 implementation
        // DataSource::S3Bucket { bucket, prefix } => load_from_s3(bucket, prefix),
        // Add other source types here
        // _ => Err(SourceError::UnsupportedSource), // Handle unsupported types
    }
}

fn load_from_local_path(path: &Path) -> Result<Vec<PipeImageData>, SourceError> {
    log_pipe_event(
        "DataSource",
        &path.display().to_string(),
        "INFO",
        "Starting load from local path.",
    );
    let mut batch = Vec::new();
    let mut load_errors: u32 = 0;

    let entries = fs::read_dir(path)?; // Propagate IO errors for reading dir itself

    for entry_result in entries {
        let entry = match entry_result {
            Ok(e) => e,
            Err(e) => {
                log_pipe_event(
                    "DataSource",
                    &path.display().to_string(),
                    "ERROR",
                    &format!("Failed reading directory entry: {}", e),
                );
                load_errors += 1;
                continue; // Skip this entry
            }
        };

        let file_path = entry.path();
        if file_path.is_file() {
            let image_id = file_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            // Attempt to read, decode, and create PipeImageData
            match ImageReader::open(&file_path)
                .map_err(|e| e.to_string()) // Convert IO error to string
                .and_then(|r| r.with_guessed_format().map_err(|e| e.to_string())) // Convert format error
                .and_then(|r| {
                    let format = r.format().unwrap_or(ImageFormat::Png); // Get format before decode
                    r.decode()
                        .map(|img| (img, format))
                        .map_err(|e| e.to_string()) // Convert decode error
                }) {
                Ok((image, image_format)) => {
                    // Successfully loaded and decoded
                    let metadata: ImageMetadata = HashMap::from([
                        ("original_filename".to_string(), json!(image_id.clone())),
                        ("original_width".to_string(), json!(image.width())),
                        ("original_height".to_string(), json!(image.height())),
                        (
                            "source_path".to_string(),
                            json!(file_path.to_string_lossy()),
                        ),
                    ]);
                    batch.push(PipeImageData {
                        id: image_id,
                        image,
                        metadata,
                        original_format: image_format,
                    });
                }
                Err(e) => {
                    log_pipe_event(
                        "DataSource",
                        &image_id,
                        "ERROR",
                        &format!("Failed to load/decode: {}", e),
                    );
                    load_errors += 1;
                }
            }
        }
    }

    log_pipe_event(
        "DataSource",
        &path.display().to_string(),
        "INFO",
        &format!(
            "Finished loading. Success: {}, Errors: {}",
            batch.len(),
            load_errors
        ),
    );

    if batch.is_empty() && load_errors > 0 {
        Err(SourceError::NoImagesLoaded)
    } else {
        Ok(batch)
    }
}

// TO-DO: S3 loader

// Placeholder for S3 loading function
// fn load_from_s3(bucket: &str, prefix: &str) -> Result<Vec<PipeImageData>, SourceError> {
//     // ... implementation using rusoto_s3 or aws-sdk-s3 ...
//     unimplemented!("S3 loading not implemented yet");
// }
