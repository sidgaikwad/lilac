use std::sync::Arc;

use tracing::{error, info};

use crate::{
    domain::{
        cluster::ports::ClusterRepository,
        queue::ports::QueueRepository,
        training_job::{models::ResourceRequirements, ports::TrainingJobRepository},
    },
    outbound::scheduler::agent_adapter::AgentSchedulerAdapter,
};
use chrono::Utc;

pub struct SchedulerService {
    job_repo: Arc<dyn TrainingJobRepository>,
    queue_repo: Arc<dyn QueueRepository>,
    cluster_repo: Arc<dyn ClusterRepository>,
    agent_adapter: Arc<AgentSchedulerAdapter>,
}

impl SchedulerService {
    pub fn new(
        job_repo: Arc<dyn TrainingJobRepository>,
        queue_repo: Arc<dyn QueueRepository>,
        cluster_repo: Arc<dyn ClusterRepository>,
        agent_adapter: Arc<AgentSchedulerAdapter>,
    ) -> Self {
        Self {
            job_repo,
            queue_repo,
            cluster_repo,
            agent_adapter,
        }
    }

    async fn cleanup_dead_nodes(&self) {
        info!("Running dead node cleanup...");
        match self.cluster_repo.list_all_nodes().await {
            Ok(nodes) => {
                for node in nodes {
                    let since_heartbeat = Utc::now() - node.heartbeat_timestamp;
                    if since_heartbeat > chrono::Duration::seconds(90) {
                        info!("Found dead node {}. Cleaning up.", node.id);

                        if let Some(job_id) = node.assigned_job_id {
                            info!("Re-queueing job {} from dead node {}", job_id, node.id);
                            if let Err(e) = self.job_repo.reset_job_status(&job_id).await {
                                error!("Failed to re-queue job {}: {}", job_id, e);
                            }
                        }

                        if let Err(e) = self.cluster_repo.delete_cluster_node(&node.id).await {
                            error!("Failed to delete dead node {}: {}", node.id, e);
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to list nodes for cleanup: {}", e);
            }
        }
    }

    pub async fn run_cycle(&self) {
        info!("Starting scheduler cycle");

        self.cleanup_dead_nodes().await;

        let queues = match self.queue_repo.get_all_queues_sorted().await {
            Ok(q) => q,
            Err(e) => {
                error!("Failed to fetch queues: {}", e);
                return;
            }
        };

        info!("Processing {} queues", queues.len());

        for queue in queues {
            let queued_jobs = match self.job_repo.get_queued_jobs_for_queue(&queue.id).await {
                Ok(jobs) => jobs,
                Err(e) => {
                    error!("Failed to fetch jobs for queue {}: {}", queue.id, e);
                    continue; // Move to the next queue
                }
            };

            if queued_jobs.is_empty() {
                continue;
            }

            info!(
                "Found {} queued jobs in queue '{}'",
                queued_jobs.len(),
                queue.name
            );

            for job in queued_jobs {
                info!("Processing job {}", job.id);
                let mut scheduled = false;

                for cluster_id in &queue.cluster_targets {
                    match self
                        .agent_adapter
                        .find_and_allocate_job(&job.id, cluster_id, &job.resource_requirements)
                        .await
                    {
                        Ok(Some(node_id)) => {
                            info!(
                                "Successfully allocated job {} to node {}",
                                job.id, node_id
                            );
                            if let Err(e) = self.job_repo.mark_as_starting(&job.id, &node_id).await
                            {
                                error!("Failed to update job {} status: {}", job.id, e);
                            }
                            scheduled = true;
                            break; // Break from cluster loop, move to next job
                        }
                        Ok(None) => {
                            // This is the expected case when no node is found, just info log.
                            info!(
                                "No suitable node found for job {} on cluster {}",
                                job.id, cluster_id
                            );
                        }
                        Err(e) => {
                            // This is an unexpected error during the node search.
                            error!(
                                "Error finding suitable node for job {} on cluster {}: {}",
                                job.id, cluster_id, e
                            );
                        }
                    }
                }

                if !scheduled {
                    info!(
                        "Could not schedule job {} on any cluster in queue '{}'",
                        job.id, queue.name
                    );
                }
            }
        }
        info!("Scheduler cycle finished");
    }
}
