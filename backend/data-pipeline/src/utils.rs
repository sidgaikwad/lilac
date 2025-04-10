//! Contains utility functions shared across different pipes

//! Contains utility functions potentially shared across different parts of the pipeline.

use std::time::SystemTime; // For adding timestamps

/// A basic logging facade for pipeline events.
///
/// # Arguments
///
/// * `pipe_name` - The name of the pipe generating the log event.
/// * `image_id` - The identifier of the image being processed.
/// * `level` - The severity level of the log (e.g., "INFO", "WARN", "ERROR").
/// * `message` - The log message content.
pub fn log_pipe_event(pipe_name: &str, image_id: &str, level: &str, message: &str) {
    // Attempt to get a simple timestamp (can be refined)
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map(|d| d.as_secs()) // Simple seconds timestamp
        .unwrap_or(0); // Default to 0 if time fails

    // Basic println! formatting. Can be replaced with structured logging later.
    println!(
        "[{}][{}][{}] Image[{}]: {}",
        timestamp,
        level.to_uppercase(), // Ensure level is uppercase
        pipe_name,
        image_id,
        message
    );

    // TODO: Replace the above println! with a proper logging crate integration, e.g.:
    // tracing::event!(target: "pipeline_events", level = level, pipe = pipe_name, image_id = image_id, message);
    // or
    // log::log!(target: "pipeline_events", level_enum_from_str(level), "Pipe[{}], Image[{}]: {}", pipe_name, image_id, message);
}
