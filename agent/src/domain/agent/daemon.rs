use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time;
use uuid::Uuid;

use crate::domain::agent::models::{HeartbeatRequest, JobInfo, NodeStatus};
use crate::domain::agent::ports::{ControlPlaneApi, JobExecutor, SystemMonitor};

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
    current_job_id: Arc<Mutex<Option<Uuid>>>,
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
            heartbeat_interval: Duration::from_secs(15), // Default to 15 seconds
            current_job_id: Arc::new(Mutex::new(None)),
            node_id,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        println!("[DAEMON] Starting Lilac agent daemon...");

        // 1. On startup, get system resources. This is sent with every heartbeat.
        let resources = self.system_monitor.get_node_resources().await?;
        println!("[DAEMON] Discovered resources: {:?}", resources);

        // 2. Start the main heartbeat loop.
        let mut interval = time::interval(self.heartbeat_interval);

        loop {
            interval.tick().await;

            let reported_job_id = self.current_job_id.lock().unwrap().clone();
            let status = if reported_job_id.is_some() {
                NodeStatus::Busy
            } else {
                NodeStatus::Available
            };

            println!("[DAEMON] Sending heartbeat... Status: {:?}, Reported Job: {:?}", status, reported_job_id);

            let request = HeartbeatRequest {
                status,
                memory_info: resources.memory_mb,
                cpu_info: resources.cpu.clone(),
                gpu_info: resources.gpus.first().cloned(), // TODO: Handle multiple GPUs
                job_info: Some(JobInfo {
                    current_job_id: reported_job_id,
                }),
            };

            let response = self
                .control_plane
                .send_heartbeat(self.node_id, request)
                .await;

            match response {
                Ok(response) => {
                    if let Some(assigned_job) = response.assigned_job {
                        println!("[DAEMON] Assigned job with ID: {}", assigned_job.id);

                        // Set the new job ID
                        *self.current_job_id.lock().unwrap() = Some(assigned_job.id);

                        let executor = self.job_executor.clone();
                        let current_job_id_clone = self.current_job_id.clone();

                        tokio::spawn(async move {
                            let job_id = assigned_job.id;
                            match executor.run_job(assigned_job).await {
                                Ok(0) => {
                                    println!("[JOB {}] Execution finished successfully.", job_id);
                                }
                                Ok(exit_code) => {
                                    eprintln!("[JOB {}] Execution finished with a non-zero exit code: {}", job_id, exit_code);
                                }
                                Err(e) => {
                                    eprintln!("[JOB {}] Execution failed: {}", job_id, e);
                                }
                            };

                            // When the job is done, clear the current job ID.
                            *current_job_id_clone.lock().unwrap() = None;
                            println!("[DAEMON] Node is now available.");
                        });
                    }
                }
                Err(e) => {
                    eprintln!("[DAEMON] Error sending heartbeat: {}. Will retry.", e);
                }
            }
        }
    }
}