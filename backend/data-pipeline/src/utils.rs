//! src/utils.rs
//! Contains utility functions potentially shared across different parts of the pipeline.

use std::time::{SystemTime, UNIX_EPOCH};
pub fn log_pipe_event(pipe_name: &str, context_id: &str, level: &str, message: &str) {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    println!(
        "[{}][{}][{}] Context[{}]: {}",
        timestamp,
        level.to_uppercase(),
        pipe_name,
        context_id,
        message
    );

    // TODO: Replace the above println! with a proper logging crate integration.
}
