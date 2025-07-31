use std::sync::Arc;

use tracing::debug;

use crate::domain::{
    cluster::{
        models::{ClusterId, NodeId, NodeStatus},
        ports::ClusterRepository,
    },
    training_job::models::{JobId, ResourceRequirements},
};

#[derive(Clone)]
pub struct AgentSchedulerAdapter {
    cluster_repo: Arc<dyn ClusterRepository>,
}

impl AgentSchedulerAdapter {
    pub fn new(cluster_repo: Arc<dyn ClusterRepository>) -> Self {
        Self { cluster_repo }
    }

    pub async fn find_and_allocate_job(
        &self,
        job_id: &JobId,
        cluster_id: &ClusterId,
        requirements: &ResourceRequirements,
    ) -> Result<Option<NodeId>, anyhow::Error> {
        let nodes = self.cluster_repo.list_cluster_nodes(cluster_id).await?;

        let suitable_node = nodes.into_iter().find(|node| {
            if node.node_status != NodeStatus::Available || node.assigned_job_id.is_some() {
                return false;
            }

            let cpu_ok = node.cpu.millicores >= requirements.cpu_millicores;
            let mem_ok = node.memory_mb >= requirements.memory_mb;
            let gpu_ok = match &requirements.gpus {
                None => true, // Job does not require GPUs.
                Some(req_gpu) => match &node.gpu {
                    Some(node_gpu) => node_gpu.count >= req_gpu.count, // Node has GPUs, check count.
                    None => false, // Job requires GPUs, but node has none.
                },
            };

            cpu_ok && mem_ok && gpu_ok
        });

        if let Some(node) = suitable_node {
            debug!("Found suitable node {} for job {}", node.id, job_id);
            self.cluster_repo
                .assign_job_to_node(&node.id, job_id)
                .await?;
            Ok(Some(node.id))
        } else {
            Ok(None)
        }
    }
}