use std::sync::Arc;

use tracing::{error, info, warn};

use super::{
    manager::ComputePlatformManager,
    models::{ClusterSnapshot, NodeSnapshot},
};
use crate::domain::training_job::{
    models::{ResourceRequirements, TrainingJobStatus},
    ports::TrainingJobRepository,
};

pub struct SchedulerService {
    platform_manager: Arc<ComputePlatformManager>,
    job_repo: Arc<dyn TrainingJobRepository>,
}

impl SchedulerService {
    pub fn new(
        platform_manager: Arc<ComputePlatformManager>,
        job_repo: Arc<dyn TrainingJobRepository>,
    ) -> Self {
        Self {
            platform_manager,
            job_repo,
        }
    }

    pub async fn run_cycle(&self) {
        info!("Starting scheduler cycle");

        let queued_jobs = match self.job_repo.get_queued_jobs_with_targets().await {
            Ok(jobs) => jobs,
            Err(e) => {
                error!("Failed to fetch queued jobs: {}", e);
                return;
            }
        };

        info!("Found {} queued jobs", queued_jobs.len());

        for job_with_targets in queued_jobs {
            let job = job_with_targets.job;
            info!("Processing job {}", job.id);

            for target in job_with_targets.targets {
                info!(
                    "Attempting to schedule job {} on cluster {}",
                    job.id, target.cluster_id
                );

                let plugin = match self
                    .platform_manager
                    .get_platform_for_cluster(&target.cluster_id)
                    .await
                {
                    Ok(p) => p,
                    Err(e) => {
                        warn!(
                            "Failed to get platform for cluster {}: {}. Skipping target.",
                            target.cluster_id, e
                        );
                        continue;
                    }
                };

                let snapshot = match plugin.get_cluster_snapshot(&target.cluster_id).await {
                    Ok(s) => s,
                    Err(e) => {
                        warn!(
                            "Failed to get snapshot for cluster {}: {}. Skipping target.",
                            target.cluster_id, e
                        );
                        continue;
                    }
                };

                if let Some(node) = find_suitable_node(&snapshot, &job.resource_requirements) {
                    info!(
                        "Found suitable node {} for job {} on cluster {}",
                        node.name, job.id, target.cluster_id
                    );

                    match plugin
                        .allocate_job(&job, &target.cluster_id, &node.name)
                        .await
                    {
                        Ok(_) => {
                            info!("Successfully allocated job {} to node {}", job.id, node.name);
                            if let Err(e) = self
                                .job_repo
                                .update_status(job.id, TrainingJobStatus::Starting)
                                .await
                            {
                                error!("Failed to update job {} status: {}", job.id, e);
                            }
                            // Break from the inner loop to move to the next job
                            break;
                        }
                        Err(e) => {
                            error!(
                                "Failed to allocate job {} to node {}: {}",
                                job.id, node.name, e
                            );
                        }
                    }
                } else {
                    info!(
                        "No suitable node found for job {} on cluster {}",
                        job.id, target.cluster_id
                    );
                }
            }
        }
        info!("Scheduler cycle finished");
    }
}

fn find_suitable_node<'a>(
    snapshot: &'a ClusterSnapshot,
    requirements: &ResourceRequirements,
) -> Option<&'a NodeSnapshot> {
    snapshot.nodes.iter().find(|node| {
        node.available_cpu_millicores >= requirements.cpu_millicores
            && node.available_memory_mb >= requirements.memory_mb
            && node.available_gpus >= requirements.gpus
    })
}