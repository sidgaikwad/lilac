use crate::{
    domain::agent::{
        models::{HeartbeatRequest, JobInfo, JobStatus},
        ports::{ControlPlaneApi, JobExecutor, SystemMonitor},
    },
};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::{task::JoinHandle, time};
use uuid::Uuid;


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
    current_job: Arc<Mutex<Option<JobInfo>>>,
    job_handle: Arc<Mutex<Option<(Uuid, JoinHandle<()>)>>>,
    node_id: Uuid,
}

impl<C, S, J> Daemon<C, S, J>
where
    C: ControlPlaneApi + Clone + 'static,
    S: SystemMonitor,
    J: JobExecutor + Clone + 'static,
{
    pub fn new(control_plane: C, system_monitor: S, job_executor: J, node_id: Uuid) -> Self {
        Self {
            control_plane: Arc::new(control_plane),
            system_monitor: Arc::new(system_monitor),
            job_executor: Arc::new(job_executor),
            heartbeat_interval: Duration::from_secs(30), // Default to 15 seconds
            current_job: Arc::new(Mutex::new(None)),
            job_handle: Arc::new(Mutex::new(None)),
            node_id,
        }
    }

    pub async fn run(self) -> Result<(), anyhow::Error> {
        println!("[DAEMON] Starting Lilac agent daemon...");

        // 1. On startup, get system resources. This is sent with every heartbeat.
        let resources = self
            .system_monitor
            .get_node_resources()
            .await
            .map_err(|e| anyhow::Error::new(e).context("Failed to get node resources"))?;
        println!("[DAEMON] Discovered resources: {:?}", resources);

        // 2. Start the main heartbeat loop.
        let mut interval = time::interval(self.heartbeat_interval);

        loop {
            interval.tick().await;

            let current_job_info = self.current_job.lock().unwrap().clone();

            println!(
                "[DAEMON] Sending heartbeat... Reported Job: {:?}",
                current_job_info
            );

            let request = HeartbeatRequest {
                memory_info: resources.memory_mb,
                cpu_info: resources.cpu.clone(),
                gpu_info: resources.gpus.first().cloned(), // TODO: Handle multiple GPUs
                job_info: current_job_info,
            };

            let response = self
                .control_plane
                .send_heartbeat(self.node_id, request)
                .await;

            match response {
                Ok(response) => {
                    let mut current_job_guard = self.current_job.lock().unwrap();
                    let assigned_job_id = response.assigned_job.as_ref().map(|j| j.id);
                    let current_job_id = current_job_guard.as_ref().map(|j| j.current_job_id);

                    // Reconciliation logic
                    if current_job_id != assigned_job_id {
                        println!(
                            "[DAEMON] Reconciliation needed. Current: {:?}, Assigned: {:?}",
                            current_job_id, assigned_job_id
                        );

                        // Cancel the current job if there is one
                        if let Some((job_id, handle)) = self.job_handle.lock().unwrap().take() {
                            println!("[DAEMON] Aborting previous job {}.", job_id);
                            handle.abort();

                            // Also stop the Docker container
                            let job_executor = self.job_executor.clone();
                            tokio::spawn(async move {
                                if let Err(e) = job_executor.stop_job(&job_id.to_string()).await {
                                    eprintln!(
                                        "[DAEMON] Error stopping job container for job {}: {}",
                                        job_id, e
                                    );
                                }
                            });
                        }

                        if let Some(assigned_job) = response.assigned_job {
                            let job_id = assigned_job.id; // Uuid is Copy, so this is fine.
                            println!("[DAEMON] Starting new job with ID: {}", job_id);
                            let new_job_info = JobInfo {
                                current_job_id: job_id,
                                status: JobStatus::Acknowledged,
                            };
                            *current_job_guard = Some(new_job_info);

                            let executor = self.job_executor.clone();
                            let current_job_clone = self.current_job.clone();
                            let job_handle_clone = self.job_handle.clone();

                            let handle = tokio::spawn(async move {
                                if let Some(job_info) = &mut *current_job_clone.lock().unwrap() {
                                    job_info.status = JobStatus::Starting;
                                }

                                // TODO: Better state updates
                                if let Some(job_info) = &mut *current_job_clone.lock().unwrap() {
                                    job_info.status = JobStatus::Running;
                                }

                                let final_status = match executor.run_job(assigned_job).await {
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

                                if let Some(job_info) = &mut *current_job_clone.lock().unwrap() {
                                    job_info.status = final_status;
                                }
                            });
                            *job_handle_clone.lock().unwrap() = Some((job_id, handle));
                        } else {
                            *current_job_guard = None;
                            println!("[DAEMON] Node is now available.");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[DAEMON] Error sending heartbeat: {}. Will retry.", e);
                }
            }
        }
    }
}