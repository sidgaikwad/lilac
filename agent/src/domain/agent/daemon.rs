use std::time::Duration;
use tokio::time;

use crate::domain::agent::models::{JobStatus, NodeStatus};
use crate::domain::agent::ports::{ControlPlaneApi, JobExecutor, SystemMonitor};
use std::sync::Arc;

pub struct Daemon<C, S, J>
where
    C: ControlPlaneApi + Clone + 'static,
    S: SystemMonitor,
    J: JobExecutor + Clone + 'static,
{
    control_plane: Arc<C>,
    system_monitor: Arc<S>,
    job_executor: Arc<J>,
    heartbeat_interval: Duration,
}

impl<C, S, J> Daemon<C, S, J>
where
    C: ControlPlaneApi + Clone + 'static,
    S: SystemMonitor,
    J: JobExecutor + Clone + 'static,
{
    pub fn new(control_plane: C, system_monitor: S, job_executor: J) -> Self {
        Self {
            control_plane: Arc::new(control_plane),
            system_monitor: Arc::new(system_monitor),
            job_executor: Arc::new(job_executor),
            heartbeat_interval: Duration::from_secs(15), // Default to 15 seconds
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        println!("[DAEMON] Starting Lilac agent daemon...");

        // 1. On startup, get system resources.
        let resources = self.system_monitor.get_node_resources().await?;
        println!("[DAEMON] Discovered resources: {:?}", resources);

        // 2. Register with the control plane.
        self.control_plane.register_node(resources).await?;
        println!("[DAEMON] Node registered successfully.");

        // 3. Start the main heartbeat loop.
        let mut interval = time::interval(self.heartbeat_interval);
        let mut current_status = NodeStatus::Available;

        loop {
            interval.tick().await;

            println!("[DAEMON] Sending heartbeat...");
            let response = self.control_plane.send_heartbeat(current_status.clone()).await?;

            if let Some(job_id) = response.assigned_job_id {
                println!("[DAEMON] Assigned job with ID: {}", job_id);
                current_status = NodeStatus::Busy;

                let job_details = self.control_plane.get_job_details(job_id).await?;
                
                // We spawn the job execution in a separate task so it doesn't
                // block the main heartbeat loop.
                let executor = self.job_executor.clone(); // Requires JobExecutor to be Clone
                let control_plane_clone = self.control_plane.clone(); // Requires ControlPlaneApi to be Clone
                
                tokio::spawn(async move {
                    let final_status = match executor.run_job(job_details).await {
                        Ok(0) => {
                            println!("[JOB {}] Execution finished successfully.", job_id);
                            JobStatus::Succeeded
                        }
                        Ok(exit_code) => {
                            eprintln!("[JOB {}] Execution finished with a non-zero exit code: {}", job_id, exit_code);
                            JobStatus::Failed
                        }
                        Err(e) => {
                            eprintln!("[JOB {}] Execution failed: {}", job_id, e);
                            JobStatus::Failed
                        }
                    };

                    let _ = control_plane_clone
                        .update_job_status(job_id, final_status)
                        .await;
                });

            } else {
                // If no job is assigned, we are available.
                current_status = NodeStatus::Available;
            }
        }
    }
}